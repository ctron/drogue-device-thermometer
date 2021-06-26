use crate::reader::AdcReader;
use core::future::Future;
use core::pin::Pin;
use drogue_device::{drivers::led::*, *};
use embassy_stm32::adc;
use embedded_hal::digital::v2::{StatefulOutputPin, ToggleableOutputPin};
use libm::log;

#[derive(Clone, Copy)]
pub enum Command {
    Toggle,
}

pub struct AppInitConfig<L1, ADC, ADCP1, ADCP2>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC: adc::Instance + Sized,
    ADCP1: adc::AdcPin<ADC> + Sized,
    ADCP2: adc::AdcPin<ADC> + Sized,
{
    pub user_led: Led<L1>,
    pub reader: AdcReader<'static, ADC, ADCP1, ADCP2>,
}

pub struct App<L1, ADC, ADCP1, ADCP2>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC: adc::Instance + Sized,
    ADCP1: adc::AdcPin<ADC> + Sized,
    ADCP2: adc::AdcPin<ADC> + Sized,
{
    config: AppInitConfig<L1, ADC, ADCP1, ADCP2>,
}

impl<L1, ADC, ADCP1, ADCP2> App<L1, ADC, ADCP1, ADCP2>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC: adc::Instance + Sized,
    ADCP1: adc::AdcPin<ADC> + Sized,
    ADCP2: adc::AdcPin<ADC> + Sized,
{
    pub fn new(config: AppInitConfig<L1, ADC, ADCP1, ADCP2>) -> Self {
        Self { config }
    }
}

impl<L1, ADC, ADCP1, ADCP2> Unpin for App<L1, ADC, ADCP1, ADCP2>
where
    L1: StatefulOutputPin + ToggleableOutputPin,
    ADC: adc::Instance,
    ADCP1: adc::AdcPin<ADC>,
    ADCP2: adc::AdcPin<ADC>,
{
}

impl<L1, ADC, ADCP1, ADCP2> Actor for App<L1, ADC, ADCP1, ADCP2>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC: adc::Instance,
    ADCP1: adc::AdcPin<ADC> + 'static,
    ADCP2: adc::AdcPin<ADC> + 'static,
{
    #[rustfmt::skip]
    type Message<'m> = Command;
    #[rustfmt::skip]
    type OnStartFuture<'m> = impl Future<Output = ()> + 'm;

    fn on_start(mut self: Pin<&mut Self>) -> Self::OnStartFuture<'_> {
        async move {
            self.config.user_led.on().ok();
        }
    }

    type OnMessageFuture<'m> = impl Future<Output = ()> + 'm;

    fn on_message(
        mut self: Pin<&mut Self>,
        message: Self::Message<'_>,
    ) -> Self::OnMessageFuture<'_> {
        match message {
            Command::Toggle => {
                self.config.user_led.toggle().ok();
                let preset = self.config.reader.read_preset() >> 4;
                let probe = self.config.reader.read_probe();
                let temp = steinhart(probe);
                defmt::info!(
                    "Preset: {}, Probe: {}, Temperature: {} °C",
                    preset,
                    probe,
                    temp
                );
            }
        }
        async {}
    }
}

/// Convert raw ADC value to degree C using Steinhart
fn steinhart(v: u16) -> f64 {
    const SERIESRESISTOR: f64 = 100_000.0; // 100kOhm
    const THERMISTORNOMINAL: f64 = 100_000.0;
    const TEMPERATURENOMINAL: f64 = 25.0;
    const BCOEFFICIENT: f64 = 3950.0;

    const MAX_VALUE: f64 = 4095.0; // 12bit

    let mut v: f64 = MAX_VALUE / v as f64;
    v = SERIESRESISTOR / v;

    let mut steinhart = v / THERMISTORNOMINAL; // (R/Ro)
    steinhart = log(steinhart); // ln(R/Ro)
    steinhart /= BCOEFFICIENT; // 1/B * ln(R/Ro)
    steinhart += 1.0 / (TEMPERATURENOMINAL + 273.15); // + (1/To)
    steinhart = 1.0 / steinhart; // Invert
    steinhart -= 273.15; // convert absolute

    // return

    steinhart
}
