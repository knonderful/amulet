use crate::geom::{Point, Size, Vector};
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn from_xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::new((x, y).into(), (w, h).into())
    }

    pub fn from_size(size: Size) -> Self {
        Self::new(Point::zero(), size)
    }

    pub fn limit(&self) -> Point {
        self.origin + self.size.as_vector()
    }

    pub fn translate(&self, vector: Vector) -> Self {
        Self::new(self.origin + vector, self.size)
    }

    pub fn clip(&self, vector: Vector) -> Self {
        let mut origin = self.origin;
        let mut size = self.size;

        if vector.x < 0 {
            size.width += vector.x;
        } else {
            size.width -= vector.x;
            origin.x += vector.x;
        }

        if vector.y < 0 {
            size.height += vector.y;
        } else {
            size.height -= vector.y;
            origin.y += vector.y;
        }

        Self::new(origin, size.fix())
    }

    pub fn resize(&self, size: Size) -> Self {
        Self::new(self.origin, size)
    }

    pub fn resize_clipped(&self, size: Size) -> Self {
        Self::new(self.origin, self.size.min(size))
    }

    pub fn contains(&self, point: Point) -> bool {
        if point.x < self.origin.x || point.y < self.origin.y {
            return false;
        }

        let limit = self.limit();
        point.x < limit.x && point.y < limit.y
    }

    pub fn inflate(&self, width: i32, height: i32) -> Self {
        let x = self.origin.x - width;
        let y = self.origin.y - height;
        let w = self.size.width + 2 * width;
        let h = self.size.height + 2 * height;
        Self::from_xywh(x, y, w, h)
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from((x, y, w, h): (i32, i32, i32, i32)) -> Self {
        Self::from_xywh(x, y, w, h)
    }
}

impl From<Rect> for (i32, i32, i32, i32) {
    fn from(value: Rect) -> Self {
        let (x, y) = value.origin.into();
        let (w, h) = value.size.into();
        (x, y, w, h)
    }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{origin: {:?}, size: {:?}}}", self.origin, self.size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let rect = Rect::from_xywh(10, 20, 30, 5);
        assert!(rect.contains((10, 20).into()));
        assert!(rect.contains((11, 20).into()));
        assert!(rect.contains((10, 21).into()));
        assert!(rect.contains((39, 24).into()));
        assert!(!rect.contains((9, 20).into()));
        assert!(!rect.contains((10, 19).into()));
        assert!(!rect.contains((40, 24).into()));
        assert!(!rect.contains((39, 25).into()));
    }

    #[test]
    fn test_clip() {
        let rect = Rect::from_xywh(10, 20, 30, 5);
        assert_eq!(Rect::from_xywh(12, 23, 28, 2), rect.clip((2, 3).into()));
        assert_eq!(Rect::from_xywh(10, 23, 28, 2), rect.clip((-2, 3).into()));
        assert_eq!(Rect::from_xywh(10, 20, 28, 2), rect.clip((-2, -3).into()));
        assert_eq!(Rect::from_xywh(12, 20, 28, 2), rect.clip((2, -3).into()));
    }

    #[test]
    fn test_inflate() {
        let rect = Rect::from_xywh(10, 20, 30, 5);
        assert_eq!(Rect::from_xywh(8, 17, 34, 11), rect.inflate(2, 3));
        assert_eq!(Rect::from_xywh(12, 17, 26, 11), rect.inflate(-2, 3));
        assert_eq!(Rect::from_xywh(8, 23, 34, -1), rect.inflate(2, -3));
        assert_eq!(Rect::from_xywh(12, 23, 26, -1), rect.inflate(-2, -3));
    }
}
