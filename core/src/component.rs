use crate::math::Size as MathSize;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::EnumCount, strum::EnumIter)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl TryFrom<sdl2::mouse::MouseButton> for MouseButton {
    type Error = ();

    fn try_from(value: sdl2::mouse::MouseButton) -> Result<Self, Self::Error> {
        use sdl2::mouse::MouseButton as MB;
        let out = match value {
            MB::Unknown | MB::X1 | MB::X2 => return Err(()),
            MB::Left => MouseButton::Left,
            MB::Middle => MouseButton::Middle,
            MB::Right => MouseButton::Right,
        };
        Ok(out)
    }
}

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

pub trait Position {
    fn position(&self) -> Point;
}

impl<T> Position for T
where
    T: Deref,
    <T as Deref>::Target: Position,
{
    fn position(&self) -> Point {
        self.deref().position()
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

pub struct Pos<C> {
    pos: Point,
    inner: C,
}

impl<C> Pos<C> {
    pub fn new(pos: Point, inner: C) -> Self {
        Self { pos, inner }
    }
}

impl<C> Inner for Pos<C> {
    type Component = C;

    fn inner(&self) -> &Self::Component {
        &self.inner
    }
}

impl<C> InnerMut for Pos<C> {
    type Component = C;

    fn inner_mut(&mut self) -> &mut Self::Component {
        &mut self.inner
    }
}

impl<C> HandleEvent for Pos<C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let event = match event {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos - self.pos),
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos - self.pos)
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos - self.pos)
            }
            other => other,
        };

        self.inner.handle_event(state, event)
    }
}

impl<C> Size for Pos<C>
where
    C: Size,
{
    fn size(&self) -> MathSize {
        self.inner.size()
    }
}

impl<C> Position for Pos<C> {
    fn position(&self) -> Point {
        self.pos
    }
}

impl<C> Render for Pos<C>
where
    C: Render,
{
    type State<'a> = C::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        (dest, constraints): (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let Some(constraints) = constraints.clip_topleft(self.pos) else {
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
