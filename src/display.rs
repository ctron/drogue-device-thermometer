use core::future::Future;
use core::pin::Pin;
use drogue_device::Actor;
use embedded_hal::blocking::i2c::Write;
use ssd1306::mode::BasicMode;
use ssd1306::prelude::I2CInterface;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize;
use ssd1306::Ssd1306;

pub struct Ssd1306Driver<'a, I2C, SIZE, E>
where
    I2C: Write<Error = E> + 'static,
    SIZE: DisplaySize,
    E: 'static,
{
    // state: Option<DriverState<I2C, E>>,
    display: Ssd1306<I2CInterface<I2C>, SIZE, BasicMode>,
    _phantom: core::marker::PhantomData<&'a I2C>,
}

impl<'a, I2C, SIZE, E> Ssd1306Driver<'a, I2C, SIZE, E>
where
    I2C: Write<Error = E> + 'static,
    SIZE: DisplaySize,
    E: 'static,
{
    pub fn new(i2c: I2C, size: SIZE, rotation: DisplayRotation) -> Self {
        let display = ssd1306::Ssd1306::new(ssd1306::I2CDisplayInterface::new(i2c), size, rotation);
        Self {
            display,
            _phantom: core::marker::PhantomData,
        }
    }
}

pub trait DisplayDriver {}

pub enum DisplayRequest<'m> {
    Show(&'m str),
}

pub struct DisplayActor<D>
where
    D: DisplayDriver + 'static,
{
    driver: D,
}

impl<D> DisplayActor<D>
where
    D: DisplayDriver + 'static,
{
    pub fn new(driver: D) -> Self {
        Self { driver }
    }
}

impl<D> Actor for DisplayActor<D>
where
    D: DisplayDriver + 'static,
{
    type Configuration = ();

    #[rustfmt::skip]
    type Message<'m> where D: 'm = DisplayRequest<'m>;
    type Response = Result<(), ()>;

    #[rustfmt::skip]
    type OnStartFuture<'m> where D: 'm = impl Future<Output = ()> + 'm;
    fn on_start(self: Pin<&mut Self>) -> Self::OnStartFuture<'_> {
        async move {}
    }

    #[rustfmt::skip]
    type OnMessageFuture<'m> where D: 'm = impl Future<Output = Self::Response> + 'm;
    fn on_message(self: Pin<&mut Self>, message: Self::Message<'_>) -> Self::OnMessageFuture<'_> {
        async move {
            match message {
                Self::Message::<'_>::Show(str) => {}
            }
            Ok(())
        }
    }
}
