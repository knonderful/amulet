pub trait LossyFrom<T>: Sized {
    fn lossy_from(value: T) -> Self;
}

pub trait LossyInto<T>: Sized {
    fn lossy_into(self) -> T;
}

impl<T, U> LossyInto<U> for T
where
    U: LossyFrom<T>,
{
    fn lossy_into(self) -> U {
        U::lossy_from(self)
    }
}

impl LossyFrom<i32> for u32 {
    fn lossy_from(value: i32) -> Self {
        Self::try_from(value).unwrap_or(0)
    }
}

impl LossyFrom<u32> for i32 {
    fn lossy_from(value: u32) -> Self {
        Self::try_from(value).unwrap_or(i32::MAX)
    }
}

impl<T, U> LossyFrom<(T, T)> for (U, U)
where
    U: LossyFrom<T>,
{
    fn lossy_from((a, b): (T, T)) -> Self {
        (a.lossy_into(), b.lossy_into())
    }
}
