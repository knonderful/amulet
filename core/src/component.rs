use crate::geom::{Point, Rect, Size, Vector};
use crate::mouse::Button;
use crate::VuiResult;
pub use frame::Frame;
pub use mouse_sensor::{MouseSensor, MouseSensorState};
use paste::paste;
pub use position::Position;

mod frame;
mod mouse_sensor;
mod position;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FramedPosition {
    absolute_position: Point,
    frame_rect: Rect,
}

impl FramedPosition {
    pub fn new(pos: Point, frame_rect: Rect) -> Self {
        Self {
            absolute_position: pos,
            frame_rect,
        }
    }

    pub fn clip(self, vector: Vector) -> Self {
        Self {
            absolute_position: self.absolute_position,
            frame_rect: self.frame_rect.clip(vector),
        }
    }

    pub fn resize(self, size: Size) -> Self {
        Self {
            absolute_position: self.absolute_position,
            frame_rect: self.frame_rect.resize(size),
        }
    }

    pub fn is_hit(&self) -> bool {
        self.frame_rect.contains(self.absolute_position)
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

    pub fn resize(self, size: Size) -> Self {
        match self {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos.resize(size)),
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos.resize(size))
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos.resize(size))
            }
            other => other,
        }
    }
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

    pub fn clip(&self, vector: Vector) -> Self {
        Self::new(self.clip_rect.clip(vector))
    }
    pub fn resize(&self, size: Size) -> Self {
        Self::new(self.clip_rect.resize(size))
    }
}

pub trait HandleEvent {
    type State<'a>;

    fn handle_event(
        &self,
        #[allow(unused)] state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        Ok(event)
    }
}

pub trait Render<R> {
    type State<'a>;

    fn render(
        &self,
        #[allow(unused)] state: Self::State<'_>,
        constraints: RenderConstraints,
        #[allow(unused)] render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        Ok(constraints)
    }
}

macro_rules! impl_tuple_handle_event {
    ( () ) => {};
    ( ( $t0:ident $(, $tx:ident)* ) ) => {
        impl<$t0, $($tx,)*> HandleEvent for ($t0, $($tx,)*) where $t0 : HandleEvent, $($tx : HandleEvent,)* {
            type State<'a> = ($t0::State<'a>, $($tx::State<'a>,)*);

            fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<ComponentEvent> {
                paste!{
                    let ([<$t0:lower>], $([<$tx:lower>],)*) = self;
                    let ([<$t0:lower _state>], $([<$tx:lower _state>],)*) = state;

                    let event = [<$t0:lower>].handle_event([<$t0:lower _state>], event)?;
                    $(
                    let event = [<$tx:lower>].handle_event([<$tx:lower _state>], event)?;
                    )*
                }
                Ok(event)
            }
        }

        impl_tuple_handle_event! { ($($tx),*) }
    };
}

macro_rules! impl_tuple_render {
    ( () ) => {};
    ( ( $t0:ident $(, $tx:ident)* ) ) => {
        impl<RdrCtx, $t0, $($tx,)*> Render<RdrCtx> for ($t0, $($tx,)*) where $t0 : Render<RdrCtx>, $($tx : Render<RdrCtx>,)* {
            type State<'a> = ($t0::State<'a>, $($tx::State<'a>,)*);

            fn render(&self, state: Self::State<'_>, constraints: RenderConstraints, render_ctx: &mut RdrCtx) -> VuiResult<RenderConstraints> {
                paste!{
                    let ([<$t0:lower>], $([<$tx:lower>],)*) = self;
                    let ([<$t0:lower _state>], $([<$tx:lower _state>],)*) = state;

                    let constraints = [<$t0:lower>].render([<$t0:lower _state>], constraints, render_ctx)?;
                    $(
                    let constraints = [<$tx:lower>].render([<$tx:lower _state>], constraints, render_ctx)?;
                    )*
                }
                Ok(constraints)
            }

        }

        impl_tuple_render! { ($($tx),*) }
    };
}

impl_tuple_handle_event! {(A, B, C, E, F, G, H, I, J, K)}
impl_tuple_render! {(A, B, C, E, F, G, H, I, J, K)}

pub trait SizeAttr {
    fn size(&self) -> Size;
}

pub trait PositionAttr {
    fn position(&self) -> Point;
}