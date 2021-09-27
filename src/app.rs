use crate::data::Telemetry;
use crate::reader::AdcReader;
use core::future::Future;
use drogue_device::traits::led::Led;
use drogue_device::{drivers::led::*, *};
use embassy_stm32::adc;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_hal::digital::v2::{StatefulOutputPin, ToggleableOutputPin};
use libm::{fabs, log};

#[cfg(feature = "display")]
use crate::display::{DisplayActor, DisplayCommand, DisplayDriver};

#[derive(Clone, Copy)]
pub enum Command {
    Toggle,
}

pub struct AppInitConfig<L1, ADC, ADCP1, ADCP2>
where
    // L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    L1: Led,
    ADC: adc::Instance + Sized,
    ADCP1: adc::AdcPin<ADC> + Sized,
    ADCP2: adc::AdcPin<ADC> + Sized,
{
    pub user_led: L1,
    pub reader: AdcReader<'static, ADC, ADCP1, ADCP2>,
}

pub struct AppConfig<'a, DPY>
where
    DPY: DisplayDriver<Color = BinaryColor> + 'static,
{
    #[cfg(feature = "display")]
    pub display: Address<'a, DisplayActor<DPY>>,
}

const MEAS: usize = 10;

pub struct App<L1, ADC, ADCP1, ADCP2, DPY>
where
    // L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    L1: Led,
    ADC: adc::Instance + Sized,
    ADCP1: adc::AdcPin<ADC> + Sized,
    ADCP2: adc::AdcPin<ADC> + Sized,
    DPY: DisplayDriver<Color = BinaryColor> + 'static,
{
    config: AppInitConfig<L1, ADC, ADCP1, ADCP2>,
    ctx: Option<AppConfig<'static, DPY>>,

    temps: usize,
    temp_pos: usize,
    temp_buffer: [f64; MEAS],
    temp_last: Option<f64>,
}

impl<L1, ADC, ADCP1, ADCP2, DPY> App<L1, ADC, ADCP1, ADCP2, DPY>
where
    // L1: StatefulOutputPin + ToggleableOutputPin + 'static,
    L1: Led,
    ADC: adc::Instance + Sized,
    ADCP1: adc::AdcPin<ADC> + Sized,
    ADCP2: adc::AdcPin<ADC> + Sized,
    DPY: DisplayDriver<Color = BinaryColor> + 'static,
{
    pub fn new(config: AppInitConfig<L1, ADC, ADCP1, ADCP2>) -> Self {
        Self {
            config,
            ctx: None,
            temp_pos: 0,
            temps: 0,
            temp_buffer: [0.0; MEAS],
            temp_last: None,
        }
    }
}

impl<L1, ADC, ADCP1, ADCP2, DPY> Unpin for App<L1, ADC, ADCP1, ADCP2, DPY>
where
    // L1: StatefulOutputPin + ToggleableOutputPin,
    L1: Led,
    ADC: adc::Instance,
    ADCP1: adc::AdcPin<ADC>,
    ADCP2: adc::AdcPin<ADC>,
    DPY: DisplayDriver<Color = BinaryColor> + 'static,
{
}

impl<L1, ADC, ADCP1, ADCP2, DPY> Actor for App<L1, ADC, ADCP1, ADCP2, DPY>
where
    // L1: StatefulOutputPin + ToggleableOutputPin,
    L1: Led + 'static,
    ADC: adc::Instance,
    ADCP1: adc::AdcPin<ADC> + 'static,
    ADCP2: adc::AdcPin<ADC> + 'static,
    DPY: DisplayDriver<Color = BinaryColor> + 'static,
{
    #[rustfmt::skip]
    type Configuration = AppConfig<'static, DPY>;
    #[rustfmt::skip]
    type Message<'m> = Command;

    #[rustfmt::skip]
    type OnMountFuture<'m, M> where M: 'm = impl Future<Output = ()> + 'm;

    fn on_mount<'m, M>(
        &'m mut self,
        config: Self::Configuration,
        _address: Address<'static, Self>,
        inbox: &'m mut M,
    ) -> Self::OnMountFuture<'m, M>
    where
        M: Inbox<'m, Self> + 'm,
    {
        self.ctx.replace(config);

        self.config.user_led.on().ok();

        async move {
            loop {
                match inbox.next().await {
                    Some(mut msg) => match msg.message() {
                        Command::Toggle => {
                            self.config.user_led.toggle().ok();
                            let preset = self.config.reader.read_preset() >> 4;

                            let probe = self.config.reader.read_probe();
                            let raw_temp = probe.map(convert);
                            let temperature = raw_temp;
                            let temperature = match (temperature, self.temp_last) {
                                (Some(temp), Some(last)) if fabs(temp - last) > 0.5 => None,
                                (temp, _) => temp,
                            };
                            self.temp_last = raw_temp;
                            defmt::info!(
                                "Preset: {}, Probe: {}, Raw Temperature: {} °C, Temperature: {} °C",
                                preset,
                                probe,
                                raw_temp,
                                temperature,
                            );
                            if let Some(temperature) = temperature {
                                if self.temps < MEAS {
                                    // missing values, record and wait
                                    let pos = self.temps;
                                    self.temp_buffer[pos] = temperature;
                                    self.temps += 1;
                                } else {
                                    // enough values, add and average
                                    let pos = self.temp_pos;
                                    self.temp_buffer[pos] = temperature;
                                    self.temp_pos += 1;
                                    self.temp_pos %= MEAS;
                                }
                            };
                            let temperature = if self.temps >= MEAS {
                                let mut result = 0f64;
                                for t in self.temp_buffer {
                                    result += t;
                                }
                                Some(result / MEAS as f64)
                            } else {
                                None
                            };
                            let telemetry = Telemetry {
                                temperature,
                                preset: preset as f64,
                            };
                            #[cfg(feature = "display")]
                            if let Some(ctx) = &self.ctx {
                                ctx.display.notify(DisplayCommand::Show(telemetry)).ok();
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn convert(v: u16) -> f64 {
    const BCOEFFICIENT: f64 = 3950.0;
    const K: f64 = 273.15;
    const T0: f64 = K + 25.0;
    const R: f64 = 100_000.0 * 0.95;

    let v = v as f64 * 3.3 / 4095.0;
    let r = (v * R) / (3.3 - v);
    let t = (T0 * BCOEFFICIENT) / (T0 * log(r / R) + BCOEFFICIENT);

    t - K
}

/// Convert raw ADC value to degree C using Steinhart
fn steinhart(v: u16) -> f64 {
    const SERIESRESISTOR: f64 = 100_000.0; // 100kOhm
    const THERMISTORNOMINAL: f64 = 100_000.0;
    const TEMPERATURENOMINAL: f64 = 25.0;
    const BCOEFFICIENT: f64 = 3950.0;

    const V_0: f64 = 3.3;

    // const MAX_VALUE: f64 = 8191 as f64; // 12bit
    const MAX_VALUE: f64 = 4095.0; // 12bit

    const R_1: f64 = 100_000.0;

    let a = 283786.2;
    let b = 0.06593;
    let c = 49886.0;

    let v = v as f64;

    let voltage = (v / MAX_VALUE) * V_0;
    let t = (-1.0 / b) * (log(((R_1 * voltage) / (a * (V_0 - voltage))) - (c / a)));

    t

    /*
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
     */
}
