use crate::geom::{ComponentSize, Rect, Vector};

// TODO: Remove this
#[derive(Default)]
pub struct RenderDestination {}

#[derive(Debug, Clone)]
pub struct RenderConstraints {
    pub clip_rect: Rect,
}

impl RenderConstraints {
    pub fn new(clip_rect: Rect) -> Self {
        Self { clip_rect }
    }

    pub fn clip(&self, rect: Rect) -> Option<Self> {
        self.clip_rect.intersection(&rect).map(Self::new)
    }

    pub fn clip_topleft(&self, delta: Vector) -> Option<Self> {
        self.clip(self.clip_rect.translate(delta))
    }

    pub fn clip_size(&self, size: ComponentSize) -> Option<Self> {
        let mut rect = self.clip_rect;
        rect.set_size(size.to_i32());
        self.clip(rect)
    }
}
