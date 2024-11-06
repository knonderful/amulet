use crate::geom::{Clip, Point, Rect, Shrink, Size, Vector};
use crate::mouse::Button;
use crate::VuiResult;

mod area;
mod mouse_sensor;
mod noop;
mod position;

pub use mouse_sensor::{MouseSensor, MouseSensorState};
pub use position::Position;
pub use area::Area;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FramedPosition {
    pos: Point,
    comp_rect: Option<Rect>,
}

impl FramedPosition {
    pub fn new(pos: Point, comp_rect: Rect) -> Self {
        Self { pos, comp_rect: Some(comp_rect) }
    }

    pub fn clip(self, vector: Vector) -> Self {
        let Some(rect) = self.comp_rect else {
            return self;
        };

        Self {
            pos: self.pos,
            comp_rect: rect.clip(vector),
        }
    }

    pub fn shrink(self, size: Size) -> Self {
        let Some(rect) = self.comp_rect else {
            return self;
        };

        Self {
            pos: self.pos,
            comp_rect: Some(rect.shrink(size)),
        }
    }

    pub fn is_hit(&self) -> bool {
        self.comp_rect
            .map(|rect| rect.contains(self.pos))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentEvent {
    LoopStart,
    MouseMotion(FramedPosition),
    MouseButtonDown(Button, FramedPosition),
    MouseButtonUp(Button, FramedPosition),
}

impl ComponentEvent {
    pub fn clip(self, vector: Vector) -> Self {
        match self {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos.clip(vector)),
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos.clip(vector))
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos.clip(vector))
            }
            other => other,
        }
    }

    pub fn shrink(self, size: Size) -> Self {
        match self {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos.shrink(size)),
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos.shrink(size))
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos.shrink(size))
            }
            other => other,
        }
    }
}

pub trait HandleEvent {
    type State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()>;
}

pub trait CalculateSize {
    fn calculate_size(&self) -> Size;
}

#[derive(Debug, Clone)]
pub struct RenderConstraints {
    clip_rect: Rect,
}

impl RenderConstraints {
    pub fn new(clip_rect: Rect) -> Self {
        Self { clip_rect }
    }

    pub fn clip_rect(&self) -> Rect {
        self.clip_rect
    }
}

impl Clip for RenderConstraints {
    fn clip(&self, vector: Vector) -> Option<Self> {
        self.clip_rect.clip(vector).map(Self::new)
    }
}

impl Shrink for RenderConstraints {
    fn shrink(&self, size: Size) -> Self {
        Self::new(self.clip_rect.shrink(size))
    }
}

pub trait Render<R> {
    type State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: R,
    ) -> VuiResult<()>;
}

pub const fn component_check<T, R>()
where
    T: HandleEvent + Render<R>,
{
}

pub const fn sized_component_check<T, R>()
where
    T: HandleEvent + Render<R> + CalculateSize,
{
}
