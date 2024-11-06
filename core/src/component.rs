use crate::generator::{Generator, GeneratorMut};
use crate::math::Size as MathSize;
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
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
            MB::Right => MouseButton::Right
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
    MouseMotion(Point),
    MouseButtonDown(MouseButton, Point),
    MouseButtonUp(MouseButton, Point),
}

pub trait HandleEvent {
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()>;
}

impl<T> HandleEvent for T
where
    T: DerefMut,
    <T as Deref>::Target: HandleEvent,
{
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        self.deref_mut().handle_event(event)
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
    fn render(&self, target: (&mut RenderDestination, RenderConstraints)) -> VuiResult<()>;
}

impl<T> Render for T
where
    T: Deref,
    <T as Deref>::Target: Render,
{
    fn render(&self, target: (&mut RenderDestination, RenderConstraints)) -> VuiResult<()> {
        self.deref().render(target)
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
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        let event = match event {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos - self.pos),
            ComponentEvent::MouseButtonUp(btn, pos) => ComponentEvent::MouseButtonUp(btn, pos - self.pos),
            ComponentEvent::MouseButtonDown(btn, pos) => ComponentEvent::MouseButtonDown(btn, pos - self.pos),
        };

        self.inner.handle_event(event)
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
    fn render(
        &self,
        (dest, constraints): (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let Some(constraints) = constraints.clip_topleft(self.pos) else {
            return Ok(());
        };
        self.inner.render((dest, constraints))
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
    fn handle_event(&mut self, _event: ComponentEvent) -> VuiResult<()> {
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
    fn render(&self, mut target: (&mut RenderDestination, RenderConstraints)) -> VuiResult<()> {
        let surface = self.renderer.render(&self.text)?;
        target.blit_surface(&surface)
    }
}

pub struct View<C> {
    components: C,
}

impl<C> View<C> {
    pub fn new(components: C) -> Self {
        Self { components }
    }
}

// We need this trait so that the calling code can use `dyn ViewElement`, since Rust doesn't allow `dyn Position + Render + Size + HandleEvent`.
pub trait PositionalComponent: Position + Render + Size + HandleEvent {}
impl<T> PositionalComponent for T where T: Position + Render + Size + HandleEvent {}

impl<G> HandleEvent for View<G>
where
    G: GeneratorMut,
    <G as GeneratorMut>::Item: HandleEvent,
{
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        let mut gen_state = G::State::default();
        while let Some(c) = self.components.next_mut(&mut gen_state) {
            c.handle_event(event.clone())?;
        }
        Ok(())
    }
}

impl<G> Size for View<G>
where
    G: Generator,
    <G as Generator>::Item: Size + Position,
{
    fn size(&self) -> MathSize {
        let (mut x_min, mut y_min, mut x_max, mut y_max) = (0, 0, 0, 0);
        let mut gen_state = G::State::default();
        while let Some(comp) = self.components.next(&mut gen_state) {
            let pos = comp.position();
            let size = comp.size();
            let cur_rect = Rect::new(pos.x(), pos.y(), size.w, size.h);
            x_min = i32::min(x_min, cur_rect.x());
            y_min = i32::min(y_min, cur_rect.y());
            x_max = i32::min(x_max, cur_rect.right());
            y_max = i32::min(y_max, cur_rect.bottom());
        }

        let out = (
            u32::try_from(x_max - x_min).expect("x_max < x_min"),
            u32::try_from(y_max - y_min).expect("y_max < y_min"),
        );

        out.into()
    }
}

impl<G> Render for View<G>
where
    G: Generator,
    <G as Generator>::Item: Render,
{
    fn render(
        &self,
        (dest, constraints): (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let mut gen_state = G::State::default();
        while let Some(comp) = self.components.next(&mut gen_state) {
            comp.render((dest, constraints.clone()))?;
        }
        Ok(())
    }
}
