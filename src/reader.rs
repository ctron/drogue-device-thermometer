use embassy_stm32::adc::{Adc, AdcPin, Instance};

pub struct AdcReader<'a, ADC, P>
where
    ADC: Instance,
    P: AdcPin<ADC>,
{
    adc: Adc<'a, ADC>,
    pin: P,
}

impl<'a, ADC, P> AdcReader<'a, ADC, P>
where
    ADC: Instance,
    P: AdcPin<ADC>,
{
    pub fn new(adc: Adc<'a, ADC>, pin: P) -> Self {
        Self { adc, pin }
    }

    pub fn read(&mut self) -> u16 {
        self.adc.read(&mut self.pin)
    }
}
