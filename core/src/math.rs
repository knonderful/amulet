pub trait LossyFrom<T>: Sized {
    #[must_use]
    fn lossy_from(value: T) -> Self;
}

pub trait LossyInto<T> {
    fn lossy_into(self) -> T;
}

impl<T, U> LossyInto<U> for T
where
    U: LossyFrom<T>,
{
    /// Calls `U::lossy_from(self)`.
    ///
    /// That is, this conversion is whatever the implementation of
    /// <code>[LossyFrom]&lt;T&gt; for U</code> chooses to do.
    #[inline]
    fn lossy_into(self) -> U {
        U::lossy_from(self)
    }
}

macro_rules! impl_lossy_min {
    ($from:ident -> $to:ident) => {
        impl LossyFrom<$from> for $to {
            fn lossy_from(value: $from) -> Self {
                value.try_into().unwrap_or($to::MIN)
            }
        }
    };
}

macro_rules! impl_lossy_max {
    ($from:ident -> $to:ident) => {
        impl LossyFrom<$from> for $to {
            fn lossy_from(value: $from) -> Self {
                value.try_into().unwrap_or($to::MAX)
            }
        }
    };
}

impl_lossy_min!(i32 -> u32);
impl_lossy_max!(u32 -> i32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lossy_i32_u32() {
        fn test(from: i32, expected: u32) {
            assert_eq!(expected, u32::lossy_from(from));
            assert_eq!(expected, from.lossy_into());
        }

        test(0, 0);
        test(10230, 10230);
        test(-1, 0);
        test(i32::MIN, 0);
        test(i32::MAX, 2147483647);
    }

    #[test]
    fn test_lossy_u32_i32() {
        fn test(from: u32, expected: i32) {
            assert_eq!(expected, i32::lossy_from(from));
            assert_eq!(expected, from.lossy_into());
        }

        test(0, 0);
        test(10230, 10230);
        test(2147483647, 2147483647);
        test(2147483648, 2147483647);
        test(u32::MAX, 2147483647);
    }
}
