use sdl2::rect::{Point, Rect};
macro_rules! impl_max_value {
    ($type:ident) => {
        impl MaxValue for $type {
            fn max_value() -> Self {
                $type::MAX
            }
        }
    };
}

pub trait MaxValue {
    fn max_value() -> Self;
}

impl_max_value!(u8);
impl_max_value!(u16);
impl_max_value!(u32);
impl_max_value!(u64);
impl_max_value!(i8);
impl_max_value!(i16);
impl_max_value!(i32);
impl_max_value!(i64);

pub trait Convert<T>: Sized
where
    T: MaxValue,
{
    fn convert_or(self, default: T) -> T;

    fn convert_clipping(self) -> T {
        self.convert_or(T::max_value())
    }
}

impl<A, B> Convert<B> for A
where
    B: MaxValue,
    B: TryFrom<A>,
{
    fn convert_or(self, default: B) -> B {
        B::try_from(self).unwrap_or(default)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    pub fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}

impl From<(u32, u32)> for Size {
    fn from(tuple: (u32, u32)) -> Self {
        let (w, h) = tuple;
        Self { w, h }
    }
}

pub trait Translated {
    fn translated(&self, delta: Point) -> Self;
}

impl Translated for Rect {
    fn translated(&self, delta: Point) -> Self {
        Rect::new(self.x() + delta.x(), self.y() + delta.y(), self.width(), self.height())
    }
}

pub trait Resized {
    fn resized(&self, size: Size) -> Self;
}

impl Resized for Rect {
    fn resized(&self, size: Size) -> Self {
        Rect::new(self.x() , self.y(), size.w, size.h)
    }
}

