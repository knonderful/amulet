use euclid::{Box2D, Point2D, Size2D, Vector2D};

pub struct AmuSpace;
pub type Distance = u32;
pub type Point = Point2D<i32, AmuSpace>;
pub type Vector = Vector2D<i32, AmuSpace>;
pub type Rect = Box2D<i32, AmuSpace>;
pub type Size = Size2D<Distance, AmuSpace>;

pub trait Clip: Sized {
    fn clip(&self, vector: Vector) -> Option<Self>;
}

impl Clip for Rect {
    fn clip(&self, vector: Vector) -> Option<Self> {
        let other = self.translate(vector);
        self.intersection(&other)
    }
}

pub trait Shrink: Sized {
    fn shrink(&self, size: Size) -> Option<Self>;
}

impl Shrink for Rect {
    fn shrink(&self, size: Size) -> Option<Self> {
        if size.is_empty() {
            return None;
        }

        Some(Self::from_origin_and_size(
            self.min,
            self.size().min(size.to_i32()),
        ))
    }
}
