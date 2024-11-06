use euclid::{Box2D, Point2D, Size2D, Vector2D};

pub struct AmuSpace;
pub type Point = Point2D<i32, AmuSpace>;
pub type Vector = Vector2D<i32, AmuSpace>;
pub type Rect = Box2D<i32, AmuSpace>;
pub type Size = Size2D<u32, AmuSpace>;
pub type ComponentSize = Size2D<u32, AmuSpace>;
