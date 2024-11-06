use crate::math::LossyInto;
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
    fn shrink(&self, size: Size) -> Self;
}

impl Shrink for Rect {
    fn shrink(&self, size: Size) -> Self {
        let old_size = self.size();
        // let old_size = (old_size.width, old_size.height);
        let (new_w, new_h): (i32, i32) = (size.width.lossy_into(), size.height.lossy_into());
        let size = (
            i32::min(old_size.width, new_w),
            i32::min(old_size.height, new_h),
        );

        Self::from_origin_and_size(self.min, size.into())
    }
}
