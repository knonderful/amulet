use crate::geom::{ComponentSize, Point};
use crate::mouse::Button;
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

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

pub trait Render {
    type State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        target: (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()>;
}

impl<T> Render for T
where
    T: Deref,
    <T as Deref>::Target: Render,
{
    type State<'a> = <T::Target as Render>::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        target: (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        self.deref().render(state, target)
    }
}

pub trait TextRenderer {
    fn size_of(&self, text: &str) -> VuiResult<ComponentSize>;
    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>>;
}

impl TextRenderer for (&Font<'_, '_>, Color) {
    fn size_of(&self, text: &str) -> VuiResult<ComponentSize> {
        self.0
            .size_of(text)
            .map(ComponentSize::from)
            .map_err(Into::into)
    }

    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>> {
        self.0.render(text).blended(self.1).map_err(Into::into)
    }
}

impl TextRenderer for (Rc<Font<'_, '_>>, Color) {
    fn size_of(&self, text: &str) -> VuiResult<ComponentSize> {
        (self.0.as_ref(), self.1).size_of(text)
    }

    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>> {
        (self.0.as_ref(), self.1).render(text)
    }
}

pub struct Text<R> {
    text: Cow<'static, str>,
    renderer: R,
}

impl<R> Text<R> {
    pub fn new(text: Cow<'static, str>, renderer: R) -> Self {
        Self { text, renderer }
    }
}

impl<R> HandleEvent for Text<R>
where
    R: TextRenderer,
{
    type State<'a> = ();

    fn handle_event(&self, _state: Self::State<'_>, _event: ComponentEvent) -> VuiResult<()> {
        Ok(())
    }
}

impl<R> Size for Text<R>
where
    R: TextRenderer,
{
    fn size(&self) -> ComponentSize {
        self.renderer.size_of(&self.text).unwrap() // TODO: components should always know their size
    }
}

impl<R> Render for Text<R>
where
    R: TextRenderer,
{
    type State<'a> = ();

    fn render(
        &self,
        _state: Self::State<'_>,
        mut target: (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let surface = self.renderer.render(&self.text)?;
        target.blit_surface(&surface)
    }
}
