use crate::data::Telemetry;
use core::fmt::Write as _;
use core::future::Future;
use display_interface::DisplayError;
use drogue_device::{Actor, Address, Inbox};
use embedded_graphics::mono_font::iso_8859_1::FONT_8X13;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::Point;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, Drawable};
use embedded_hal::blocking::i2c::Write;
use heapless::String;
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::I2CInterface, rotation::DisplayRotation,
    size::DisplaySize, Ssd1306,
};

pub struct Ssd1306Driver<'a, I2C, SIZE, E>
where
    I2C: Write<Error = E> + 'static,
    SIZE: DisplaySize,
    E: 'static,
{
    // state: Option<DriverState<I2C, E>>,
    display: Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>,
    _phantom: core::marker::PhantomData<&'a I2C>,
}

impl<'a, I2C, SIZE, E> Ssd1306Driver<'a, I2C, SIZE, E>
where
    I2C: Write<Error = E> + 'static,
    SIZE: DisplaySize,
    E: 'static,
{
    pub fn new(i2c: I2C, size: SIZE, rotation: DisplayRotation) -> Self {
        let display = ssd1306::Ssd1306::new(ssd1306::I2CDisplayInterface::new(i2c), size, rotation)
            .into_buffered_graphics_mode();
        Self {
            display,
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn display(&mut self) -> &mut Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>> {
        &mut self.display
    }
}

pub trait DisplayDriver {
    type Target: DrawTarget<Color = Self::Color>;
    type Color;
    type Error;

    fn as_draw(&mut self) -> &mut Self::Target;

    fn flush(&mut self) -> Result<(), Self::Error>;
}

impl<'a, I2C, SIZE, E> DisplayDriver for Ssd1306Driver<'a, I2C, SIZE, E>
where
    I2C: Write<Error = E> + 'static,
    SIZE: DisplaySize,
    E: 'static,
{
    type Target = Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>;
    type Color = BinaryColor;
    type Error = DisplayError;

    fn as_draw(&mut self) -> &mut Self::Target {
        &mut self.display
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.display.flush()
    }
}

pub enum DisplayCommand {
    Show(Telemetry),
}

pub struct DisplayActor<D>
where
    D: DisplayDriver,
{
    driver: D,
}

impl<D> DisplayActor<D>
where
    D: DisplayDriver,
{
    pub fn new(driver: D) -> Self {
        Self { driver }
    }
}

impl<D> Unpin for DisplayActor<D> where D: DisplayDriver + 'static {}

impl<D> Actor for DisplayActor<D>
where
    D: DisplayDriver<Color = BinaryColor> + 'static,
{
    type Configuration = ();

    #[rustfmt::skip]
    type Message<'m> = DisplayCommand;

    #[rustfmt::skip]
    type OnMountFuture<'m, M> where M: 'm = impl Future<Output = ()> + 'm;

    fn on_mount<'m, M>(
        &'m mut self,
        _config: Self::Configuration,
        _address: Address<'static, Self>,
        inbox: &'m mut M,
    ) -> Self::OnMountFuture<'m, M>
    where
        M: Inbox<'m, Self> + 'm,
    {
        async move {
            loop {
                match inbox.next().await {
                    Some(mut msg) => match msg.message() {
                        Self::Message::<'_>::Show(telemetry) => {
                            self.display_telemetry(telemetry);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

impl<D> DisplayActor<D>
where
    D: DisplayDriver<Color = BinaryColor> + 'static,
{
    fn display_telemetry(&mut self, telemetry: &Telemetry) {
        {
            let display = self.driver.as_draw();
            display.clear(BinaryColor::Off).ok();

            let style = MonoTextStyle::new(&FONT_8X13, BinaryColor::On);

            let mut buf: String<32> = String::new();

            if let Some(temperature) = telemetry.temperature {
                write!(
                    &mut buf,
                    "{:.1} °C / {:.0} °C",
                    temperature, telemetry.preset
                )
                .ok();
            } else {
                write!(&mut buf, "<??> °C / {:.0} °C", telemetry.preset).ok();
            }

            embedded_graphics::text::Text::new(&buf, Point::new(1, 20), style)
                .draw(display)
                .ok();
        }

        self.driver.flush().ok();
    }
}
