use embassy_stm32::adc::{Adc, AdcPin, Instance, Resolution};

pub struct AdcReader<'a, ADC, P1, P2>
where
    ADC: Instance,
    P1: AdcPin<ADC>,
    P2: AdcPin<ADC>,
{
    adc: Adc<'a, ADC>,
    resolution_max: u32,
    preset: P1,
    probe: P2,
}

impl<'a, ADC, P1, P2> AdcReader<'a, ADC, P1, P2>
where
    ADC: Instance,
    P1: AdcPin<ADC>,
    P2: AdcPin<ADC>,
{
    pub fn new(mut adc: Adc<'a, ADC>, resolution: Resolution, preset: P1, probe: P2) -> Self {
        let resolution_max = Self::to_max_count(&resolution);
        adc.set_resolution(resolution);
        Self {
            adc,
            resolution_max,
            preset,
            probe,
        }
    }

    pub fn read_preset(&mut self) -> u16 {
        self.adc.read(&mut self.preset)
    }

    pub fn read_probe(&mut self) -> Option<u16> {
        let v = self.adc.read(&mut self.probe);
        if v as u32 >= self.resolution_max {
            None
        } else {
            Some(v)
        }
    }

    fn to_max_count(resolution: &Resolution) -> u32 {
        match resolution {
            Resolution::TwelveBit => (1 << 12) - 1,
            Resolution::TenBit => (1 << 10) - 1,
            Resolution::EightBit => (1 << 8) - 1,
            Resolution::SixBit => (1 << 6) - 1,
        }
    }
}
