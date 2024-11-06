use crate::math::Size as MathSize;
use crate::mouse::MouseButton;
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

pub mod mouse_aware;

pub trait Inner {
    type Component;
    fn inner(&self) -> &Self::Component;
}

pub trait InnerMut {
    type Component;
    fn inner_mut(&mut self) -> &mut Self::Component;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentEvent {
    LoopStart,
    MouseMotion(Point),
    MouseButtonDown(MouseButton, Point),
    MouseButtonUp(MouseButton, Point),
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
    fn size(&self) -> MathSize;
}

impl<T> Size for T
where
    T: Deref,
    <T as Deref>::Target: Size,
{
    fn size(&self) -> MathSize {
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

pub struct Position<C> {
    value: Point,
    inner: C,
}

impl<C> Position<C> {
    pub fn new(pos: Point, inner: C) -> Self {
        Self { value: pos, inner }
    }

    pub fn position(&self) -> Point {
        self.value
    }
}

impl<C> Inner for Position<C> {
    type Component = C;

    fn inner(&self) -> &Self::Component {
        &self.inner
    }
}

impl<C> InnerMut for Position<C> {
    type Component = C;

    fn inner_mut(&mut self) -> &mut Self::Component {
        &mut self.inner
    }
}

impl<C> HandleEvent for Position<C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let event = match event {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos - self.value),
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos - self.value)
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos - self.value)
            }
            other => other,
        };

        self.inner.handle_event(state, event)
    }
}

impl<C> Size for Position<C>
where
    C: Size,
{
    fn size(&self) -> MathSize {
        self.inner.size()
    }
}

impl<C> Render for Position<C>
where
    C: Render,
{
    type State<'a> = C::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        (dest, constraints): (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let Some(constraints) = constraints.clip_topleft(self.value) else {
            return Ok(());
        };
        self.inner.render(state, (dest, constraints))
    }
}

pub trait TextRenderer {
    fn size_of(&self, text: &str) -> VuiResult<MathSize>;
    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>>;
}

impl TextRenderer for (&Font<'_, '_>, Color) {
    fn size_of(&self, text: &str) -> VuiResult<MathSize> {
        self.0.size_of(text).map(MathSize::from).map_err(Into::into)
    }

    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>> {
        self.0.render(text).blended(self.1).map_err(Into::into)
    }
}

impl TextRenderer for (Rc<Font<'_, '_>>, Color) {
    fn size_of(&self, text: &str) -> VuiResult<MathSize> {
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
    fn size(&self) -> MathSize {
        self.renderer
            .size_of(&self.text)
            .map(MathSize::from)
            .unwrap_or(MathSize::new(0, 0)) // TODO: components should always know their size
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
