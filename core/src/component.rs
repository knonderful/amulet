use crate::geom::{ComponentSize, Point};
use crate::mouse::Button;
use crate::render::{RenderConstraints, RenderDestination};
use crate::VuiResult;
use std::ops::Deref;

mod mouse_sensor;
mod position;

pub use mouse_sensor::{MouseSensor, MouseSensorState};
pub use position::Position;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentEvent {
    LoopStart,
    MouseMotion(Point),
    MouseButtonDown(Button, Point),
    MouseButtonUp(Button, Point),
}

pub trait Inner {
    type Component;
    fn inner(&self) -> &Self::Component;
}

pub trait InnerMut {
    type Component;
    fn inner_mut(&mut self) -> &mut Self::Component;
}

pub trait HandleEvent {
    type State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()>;
}

impl<T> HandleEvent for T
where
    T: Deref,
    <T as Deref>::Target: HandleEvent,
{
    type State<'a> = <<T as Deref>::Target as HandleEvent>::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.deref().handle_event(state, event)
    }
}

pub trait Size {
    fn size(&self) -> ComponentSize;
}

impl<T> Size for T
where
    T: Deref,
    <T as Deref>::Target: Size,
{
    fn size(&self) -> ComponentSize {
        self.deref().size()
    }
}

pub trait Render<X> {
    type State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        target: (&mut RenderDestination, RenderConstraints),
        render_ctx: X,
    ) -> VuiResult<()>;
}

impl<T, X> Render<X> for T
where
    T: Deref,
    <T as Deref>::Target: Render<X>,
{
    type State<'a> = <T::Target as Render<X>>::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        target: (&mut RenderDestination, RenderConstraints),
        render_ctx: X,
    ) -> VuiResult<()> {
        self.deref().render(state, target, render_ctx)
    }
}
