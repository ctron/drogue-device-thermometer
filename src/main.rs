#![no_std]
#![no_main]
#![macro_use]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(concat_idents)]

use defmt_rtt as _;
use panic_probe as _;

#[cfg(feature = "wifi")]
use drogue_device::actors::wifi::{esp8266::*, *};

use drogue_device::{actors::ticker::*, drivers::led::*, *};
use embassy_stm32::{
    adc::{Adc, Resolution},
    gpio::{Level, Output},
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
use core::fmt::Write;
use cortex_m::delay::Delay;
use embassy::time::Duration;

#[cfg(feature = "display")]
mod display;

use embassy_stm32::gpio::Speed;

#[cfg(feature = "wifi")]
use drogue_device::{actors::wifi::esp8266::Esp8266Wifi, traits::ip::SocketAddress};

#[cfg(feature = "display")]
use crate::display::DisplayActor;
use embassy_stm32::adc::SampleTime;
use embassy_stm32::dma::NoDma;
#[cfg(feature = "display")]
use embassy_stm32::i2c;
use heapless::String;
#[cfg(feature = "display")]
use ssd1306::{mode::DisplayConfig, size::DisplaySize128x32};

use embassy_stm32::pac;

type Led1Pin = Output<'static, PB3>;

type MyApp = App<Led<Led1Pin>, ADC1, PA0, PA1, Ssd1306>;

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

/*
fn test() {
    embassy_stm32::Config::default().rcc(
        embassy_stm32::rcc::Config::default()
            .clock_src(embassy_stm32::rcc::ClockSrc::HSE(80.mhz().into())),
    );
}
*/

#[embassy::main]
/*
#[embassy::main(config = "embassy_stm32::Config::default().rcc(
        embassy_stm32::rcc::Config::default()
            .clock_src(embassy_stm32::rcc::ClockSrc::HSE(80.mhz().into())),
    )")]
*/
async fn main(spawner: embassy::executor::Spawner, p: Peripherals) {
    let cp = cortex_m::Peripherals::take().unwrap();

    // let pp = pac::Peripherals::take().unwrap();

    let mut delay = Delay::new(cp.SYST, 16_000_000);

    unsafe {
        pac::RCC.ccipr().modify(|w| {
            //w.i2c1sel().bits(0b11);
            //w.adcsel().bits(0b11);
            w.set_i2c1sel(0b11);
            w.set_adcsel(0b11);
        });

        pac::DBGMCU.cr().modify(|w| {
            w.set_dbg_sleep(true);
            w.set_dbg_standby(true);
            w.set_dbg_stop(true);
        });

        pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));

        pac::RCC.ahb2enr().modify(|w| {
            w.set_adcen(true);
            w.set_gpioaen(true);
            w.set_gpioben(true);
            w.set_gpiocen(true);
            w.set_gpioden(true);
            w.set_gpioeen(true);
        });

        pac::RCC.apb1enr1().modify(|w| {
            w.set_i2c1en(true);
        });
    }

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

        let enable_pin = Output::new(p.P0_09, Level::Low, Speed::Medium);
        let reset_pin = Output::new(p.P0_10, Level::Low, Speed::Medium);
    }

    #[cfg(feature = "display")]
    // let i2c = i2c::I2c::new(p.I2C1, p.PB6, p.PB7, 80.khz());
    let i2c = i2c::I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        interrupt::take!(I2C1_EV),
        NoDma, // p.DMA1_CH7,
        NoDma, // p.DMA1_CH6,
        80.khz(),
    );

    let led1 = Led::new(Output::new(p.PB3, Level::High, Speed::Low));
    // let buzzer = Output::new(p.A8, Level::High, Speed::Medium);

    let mut adc = Adc::new(p.ADC1, &mut delay);
    adc.set_sample_time(SampleTime::Cycles247_5);

    let mut reader = AdcReader::new(adc, Resolution::TwelveBit, p.PA0, p.PA1);

    let sample = reader.read_probe();
    defmt::info!("Test ADC: {}", sample);

    #[cfg(feature = "display")]
    let display = {
        defmt::info!("Init display");
        let mut display = display::Ssd1306Driver::new(
            i2c,
            ssd1306::size::DisplaySize128x32,
            ssd1306::rotation::DisplayRotation::Rotate0,
        );

        defmt::info!("Created display");

        {
            defmt::info!("Access display");
            let display = display.display();
            defmt::info!("Init display");
            match display.init() {
                Ok(_) => defmt::info!("Display initialized"),
                Err(err) => {
                    let mut str: String<32> = String::new();
                    write!(&mut str, "{:?}", err).ok();
                    defmt::warn!("Display err: {:?}", str.as_str());
                }
            }
            defmt::info!("Enable display");
            match display.set_display_on(true) {
                Ok(_) => defmt::info!("Display on"),
                Err(err) => {
                    let mut str: String<32> = String::new();
                    write!(&mut str, "{:?}", err).ok();
                    defmt::warn!("Display err: {:?}", str.as_str());
                }
            }
        }
        defmt::info!("Display ready");
        display
    };

    defmt::info!("Configure application");

    DEVICE.configure(MyDevice {
        #[cfg(feature = "wifi")]
        wifi: Esp8266Wifi::new(u, enable_pin, reset_pin),
        ticker: ActorContext::new(Ticker::new(Duration::from_millis(250), Command::Toggle)),
        app: ActorContext::new(App::new(AppInitConfig {
            user_led: led1,
            reader,
        })),
        #[cfg(feature = "display")]
        display: ActorContext::new(DisplayActor::new(display)),
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
