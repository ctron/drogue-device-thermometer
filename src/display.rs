use crate::data::Telemetry;
use core::fmt::Write as _;
use core::{
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
};
use display_interface::DisplayError;
use drogue_device::Actor;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::Point;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, Drawable};
use embedded_hal::blocking::i2c::Write;
use heapless::String;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::{
    mode::BasicMode, prelude::I2CInterface, rotation::DisplayRotation, size::DisplaySize, Ssd1306,
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
}

pub trait DisplayDriver {
    type Target: DrawTarget<Color = Self::Color>;
    type Color;

    fn as_draw(&mut self) -> &mut Self::Target;
}

impl<'a, I2C, SIZE, E> DisplayDriver for Ssd1306Driver<'a, I2C, SIZE, E>
where
    I2C: Write<Error = E> + 'static,
    SIZE: DisplaySize,
    E: 'static,
{
    type Target = Ssd1306<I2CInterface<I2C>, SIZE, BufferedGraphicsMode<SIZE>>;
    type Color = BinaryColor;

    fn as_draw(&mut self) -> &mut Self::Target {
        &mut self.display
    }
}

pub enum DisplayRequest {
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
    type Message<'m> where D: 'm = DisplayRequest;
    #[rustfmt::skip]
    type Response = Result<(), ()>;

    #[rustfmt::skip]
    type OnStartFuture<'m> where D: 'm = impl Future<Output = ()> + 'm;
    fn on_start(self: Pin<&mut Self>) -> Self::OnStartFuture<'_> {
        async move {}
    }

    #[rustfmt::skip]
    type OnMessageFuture<'m> where D: 'm = impl Future<Output = Self::Response> + 'm;

    fn on_message<'m>(
        mut self: Pin<&'m mut Self>,
        message: Self::Message<'m>,
    ) -> Self::OnMessageFuture<'m> {
        async move {
            match message {
                Self::Message::<'_>::Show(telemetry) => {
                    self.display_telemetry(telemetry);
                }
            }
            Ok(())
        }
    }
}

impl<D> DisplayActor<D>
where
    D: DisplayDriver<Color = BinaryColor> + 'static,
{
    fn display_telemetry(&mut self, telemetry: Telemetry) {
        let mut display = self.driver.as_draw();
        display.clear(BinaryColor::Off).ok();

        let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        let mut buf: String<32> = String::new();
        write!(&mut buf, "{:.1} C", telemetry.temperature);

        embedded_graphics::text::Text::new(&buf, Point::zero(), style).draw(display);
    }
}
