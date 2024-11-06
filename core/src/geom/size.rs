use crate::geom::Vector;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Hash)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn as_vector(&self) -> Vector {
        Vector::new(self.width, self.height)
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn fix(mut self) -> Self {
        self.width = self.width.max(0);
        self.height = self.height.max(0);
        self
    }

    pub fn min(&self, other: Self) -> Self {
        Self::new(self.width.min(other.width), self.height.min(other.height))
    }

    pub fn max(&self, other: Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }
}

impl From<(i32, i32)> for Size {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Size> for (i32, i32) {
    fn from(value: Size) -> Self {
        (value.width, value.height)
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl Add<Vector> for Size {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.width + rhs.x, self.height + rhs.y)
    }
}

impl Sub<Vector> for Size {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::new(self.width - rhs.x, self.height - rhs.y)
    }
}

impl Mul<i32> for Size {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}
