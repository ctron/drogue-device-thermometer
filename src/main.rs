#![no_std]
#![no_main]
#![macro_use]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(generic_associated_types)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![feature(concat_idents)]

use defmt_rtt as _;
use panic_probe as _;

use drogue_device::{
    actors::{
        ticker::*,
        wifi::{esp8266::*, *},
    },
    drivers::led::*,
    *,
};
use embassy_stm32::{
    adc::{Adc, Resolution},
    gpio::{Level, NoPin, Output},
    interrupt,
    peripherals::{ADC1, I2C1, PA0, PA1, PA12, PB0, PB3},
    time::U32Ext,
    Peripherals,
};

mod app;
mod data;
mod reader;

use crate::reader::AdcReader;
use app::*;
use cortex_m::delay::Delay;
use embassy::time::Duration;
use stm32l4::stm32l4x2 as pac;

#[cfg(feature = "display")]
use ssd1306::size::DisplaySize128x32;
#[cfg(feature = "display")]
mod display;
#[cfg(feature = "display")]
use embassy_stm32::i2c;
#[cfg(feature = "display")]
use ssd1306::rotation::DisplayRotation;

use crate::display::DisplayActor;
#[cfg(feature = "wifi")]
use drogue_device::{actors::wifi::esp8266::Esp8266Wifi, traits::ip::SocketAddress};
use embassy_stm32::gpio::Speed;

type Led1Pin = Output<'static, PB3>;

type MyApp = App<Led1Pin, ADC1, PA0, PA1, Ssd1306>;

#[cfg(feature = "wifi")]
type UART = BufferedUarte<'static, UARTE0, TIMER0>;
#[cfg(feature = "wifi")]
type ENABLE = Output<'static, PA12>;
#[cfg(feature = "wifi")]
type RESET = Output<'static, PB0>;

#[cfg(feature = "display")]
pub type Ssd1306 =
    display::Ssd1306Driver<'static, i2c::I2c<'static, I2C1>, DisplaySize128x32, i2c::Error>;

pub struct MyDevice {
    #[cfg(feature = "wifi")]
    wifi: Esp8266Wifi<UART, ENABLE, RESET>,
    app: ActorContext<'static, MyApp>,
    ticker: ActorContext<'static, Ticker<'static, MyApp>>,
    #[cfg(feature = "display")]
    display: ActorContext<'static, display::DisplayActor<Ssd1306>>,
}

static DEVICE: DeviceContext<MyDevice> = DeviceContext::new();

fn test() {
    embassy_stm32::Config::default().rcc(
        embassy_stm32::rcc::Config::default()
            .clock_src(embassy_stm32::rcc::ClockSrc::HSE(80.mhz().into())),
    );
}

#[embassy::main]
/*
#[embassy::main(config = "embassy_stm32::Config::default().rcc(
        embassy_stm32::rcc::Config::default()
            .clock_src(embassy_stm32::rcc::ClockSrc::HSE(80.mhz().into())),
    )")]
 */
async fn main(spawner: embassy::executor::Spawner, p: Peripherals) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let pp = pac::Peripherals::take().unwrap();

    let delay = Delay::new(cp.SYST, 16_000_000);

    pp.RCC.ccipr.modify(|_, w| {
        unsafe {
            w.adcsel().bits(0b11);
        }
        w
    });

    pp.DBGMCU.cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_standby().set_bit();
        w.dbg_stop().set_bit()
    });

    pp.RCC.ahb1enr.modify(|_, w| w.dma1en().set_bit());

    pp.RCC.ahb2enr.modify(|_, w| {
        w.adcen().set_bit();
        w.gpioaen().set_bit();
        w.gpioben().set_bit();
        w.gpiocen().set_bit();
        w.gpioden().set_bit();
        w.gpioeen().set_bit();
        w
    });

    defmt::info!("Starting up...");

    #[cfg(feature = "wifi")]
    {
        defmt::info!("Configure ESP");
        let mut config = uarte::Config::default();
        config.parity = uarte::Parity::EXCLUDED;
        config.baudrate = uarte::Baudrate::BAUD115200;

        static mut TX_BUFFER: [u8; 256] = [0u8; 256];
        static mut RX_BUFFER: [u8; 256] = [0u8; 256];

        let irq = interrupt::take!(UARTE0_UART0);
        let u = unsafe {
            BufferedUarte::new(
                p.UARTE0,
                p.TIMER0,
                p.PPI_CH0,
                p.PPI_CH1,
                irq,
                p.P0_13,
                p.P0_01,
                NoPin,
                NoPin,
                config,
                &mut RX_BUFFER,
                &mut TX_BUFFER,
            )
        };

        let enable_pin = Output::new(p.P0_09, Level::Low);
        let reset_pin = Output::new(p.P0_10, Level::Low);
    }

    #[cfg(feature = "display")]
    let i2c = i2c::I2c::new(p.I2C1, p.PB6, p.PB7, 200_000.hz());

    let led1 = Led::new(Output::new(p.PB3, Level::High, Speed::Low));

    let (mut adc, _) = Adc::new(p.ADC1, delay);
    adc.set_resolution(Resolution::TwelveBit);

    let reader = AdcReader::new(adc, p.PA0, p.PA1);

    DEVICE.configure(MyDevice {
        #[cfg(feature = "wifi")]
        wifi: Esp8266Wifi::new(u, enable_pin, reset_pin),
        ticker: ActorContext::new(Ticker::new(Duration::from_millis(250), Command::Toggle)),
        app: ActorContext::new(App::new(AppInitConfig {
            user_led: led1,
            reader,
        })),
        #[cfg(feature = "display")]
        display: ActorContext::new(DisplayActor::new(display::Ssd1306Driver::new(
            i2c,
            ssd1306::size::DisplaySize128x32,
            ssd1306::rotation::DisplayRotation::Rotate0,
        ))),
    });

    DEVICE
        .mount(|device| async move {
            #[cfg(feature = "display")]
            let display = device.display.mount((), spawner);
            let app = device.app.mount(AppConfig { display }, spawner);
            let ticker = device.ticker.mount(app, spawner);
            ticker.notify(TickerCommand::Start).unwrap();
        })
        .await;
}
