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

use drogue_device::{actors::ticker::*, drivers::led::*, *};
use embassy_stm32::{
    gpio::{Level, Output},
    interrupt,
    peripherals::{ADC1, PA0, PB3},
    Peripherals,
};

mod app;
mod reader;

use app::*;
use cortex_m::delay::Delay;
use embassy::time::Duration;
use embassy_stm32::adc::{Adc, Resolution};
use embassy_stm32::time::U32Ext;

use stm32l4::stm32l4x2 as pac;

#[cfg(feature = "display")]
use ssd1306::size::DisplaySize128x32;
#[cfg(feature = "display")]
mod display;
use crate::reader::AdcReader;
#[cfg(feature = "display")]
use embassy_stm32::i2c;

type Led1Pin = Output<'static, PB3>;

type MyApp = App<Led1Pin, ADC1, PA0>;

#[cfg(feature = "display")]
pub type Ssd1306<'a> =
    display::Ssd1306Driver<'a, i2c::Spi<'a, pac::I2C1>, DisplaySize128x32, i2c::Error>;

pub struct MyDevice {
    app: ActorContext<'static, MyApp>,
    ticker: ActorContext<'static, Ticker<'static, MyApp>>,
    #[cfg(feature = "display")]
    display: ActorContext<'static, Ssd1306<'static>>,
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

    #[cfg(feature = "display")]
    let i2c = i2c::I2C::new(
        p.I2C1,
        p.PB3,
        p.PA7,
        p.PA6,
        200_000.hz(),
        i2c::Config::default(),
    );

    let led1 = Led::new(Output::new(p.PB3, Level::High));

    let (mut adc, _) = Adc::new(p.ADC1, delay);
    adc.set_resolution(Resolution::TwelveBit);

    let mut set_temp = AdcReader::new(adc, p.PA0);

    DEVICE.configure(MyDevice {
        ticker: ActorContext::new(Ticker::new(Duration::from_millis(250), Command::Toggle)),
        app: ActorContext::new(App::new(AppInitConfig {
            user_led: led1,
            set_temp,
        })),
        #[cfg(feature = "display")]
        display: ActorContext::new(Ssd1306Driver::new(i2c)),
    });

    DEVICE.mount(|device| {
        let app = device.app.mount((), spawner);
        let ticker = device.ticker.mount(app, spawner);
        ticker.notify(TickerCommand::Start).unwrap();
    });
}
