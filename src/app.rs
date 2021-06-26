use crate::reader::AdcReader;
use core::future::Future;
use core::pin::Pin;
use drogue_device::{drivers::led::*, *};
use embassy_stm32::adc;
use embedded_hal::digital::v2::{StatefulOutputPin, ToggleableOutputPin};

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
                defmt::info!("Preset: {}, Probe: {}", preset, probe);
            }
        }
        async {}
    }
}
