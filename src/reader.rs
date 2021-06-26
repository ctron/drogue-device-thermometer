use embassy_stm32::adc::{Adc, AdcPin, Instance};

pub struct AdcReader<'a, ADC, P1, P2>
where
    ADC: Instance,
    P1: AdcPin<ADC>,
    P2: AdcPin<ADC>,
{
    adc: Adc<'a, ADC>,
    preset: P1,
    probe: P2,
}

impl<'a, ADC, P1, P2> AdcReader<'a, ADC, P1, P2>
where
    ADC: Instance,
    P1: AdcPin<ADC>,
    P2: AdcPin<ADC>,
{
    pub fn new(adc: Adc<'a, ADC>, preset: P1, probe: P2) -> Self {
        Self { adc, preset, probe }
    }

    pub fn read_preset(&mut self) -> u16 {
        self.adc.read(&mut self.preset)
    }

    pub fn read_probe(&mut self) -> u16 {
        self.adc.read(&mut self.probe)
    }
}
