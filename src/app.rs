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

pub struct AppInitConfig<L1, ADC1, ADC1P>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC1: adc::Instance + Sized,
    ADC1P: adc::AdcPin<ADC1> + Sized,
{
    pub user_led: Led<L1>,
    pub set_temp: AdcReader<'static, ADC1, ADC1P>,
}

pub struct App<L1, ADC1, ADC1P>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC1: adc::Instance + Sized,
    ADC1P: adc::AdcPin<ADC1> + Sized,
{
    config: AppInitConfig<L1, ADC1, ADC1P>,
}

impl<L1, ADC1, ADC1P> App<L1, ADC1, ADC1P>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC1: adc::Instance + Sized,
    ADC1P: adc::AdcPin<ADC1> + Sized,
{
    pub fn new(config: AppInitConfig<L1, ADC1, ADC1P>) -> Self {
        Self { config }
    }
}

impl<L1, ADC1, ADC1P> Unpin for App<L1, ADC1, ADC1P>
where
    L1: StatefulOutputPin + ToggleableOutputPin,
    ADC1: adc::Instance,
    ADC1P: adc::AdcPin<ADC1>,
{
}

impl<L1, ADC1, ADC1P> Actor for App<L1, ADC1, ADC1P>
where
    L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    ADC1: adc::Instance,
    ADC1P: adc::AdcPin<ADC1> + 'static,
{
    #[rustfmt::skip]
    type Message<'m> = Command;
    #[rustfmt::skip]
    type OnStartFuture<'m> = impl Future<Output = ()> + 'm;

    fn on_start<'m>(mut self: Pin<&'m mut Self>) -> Self::OnStartFuture<'m> {
        async move {
            self.config.user_led.on().ok();
        }
    }

    type OnMessageFuture<'m> = impl Future<Output = ()> + 'm;

    fn on_message<'m>(
        mut self: Pin<&'m mut Self>,
        message: Self::Message<'m>,
    ) -> Self::OnMessageFuture<'m> {
        match message {
            Command::Toggle => {
                self.config.user_led.toggle().ok();
                let v = self.config.set_temp.read() >> 4;
                defmt::info!("Set: {}", v);
            }
        }
        async {}
    }
}
