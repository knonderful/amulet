use crate::generator::{Generator, GeneratorMut};
use crate::math::Size as MathSize;
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentEvent {
    MouseMove(Point),
    MouseDown(Point), // TODO: Add button
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

impl<C> HandleEvent for Pos<C>
where
    C: HandleEvent,
{
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        let event = match event {
            ComponentEvent::MouseMove(pos) => ComponentEvent::MouseMove(pos - self.pos),
            ComponentEvent::MouseDown(pos) => ComponentEvent::MouseDown(pos - self.pos),
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

pub struct MouseAware<C> {
    inner: C,
    hover: bool,
}

impl<C> MouseAware<C> {
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            hover: false,
        }
    }
}

impl<C> Size for MouseAware<C>
where
    C: Size,
{
    fn size(&self) -> MathSize {
        self.inner.size()
    }
}

impl<C> HandleEvent for MouseAware<C>
where
    C: HandleEvent + Size,
{
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        match event {
            ComponentEvent::MouseMove(pos) => {
                let size = self.size();
                let rect = Rect::new(0, 0, size.w, size.h);
                if rect.contains_point(pos) {
                    self.hover = true;
                    println!("MOUSE MOVE");
                } else {
                    self.hover = false;
                    println!("MOUSE LEFT");
                }
            }
            ComponentEvent::MouseDown(pos) => {
                let size = self.size();
                let rect = Rect::new(0, 0, size.w, size.h);
                if rect.contains_point(pos) {
                    println!("MOUSE CLICK");
                }
            }
        }
        self.inner.handle_event(event)
    }
}

impl<C> Render for MouseAware<C>
where
    C: Render + Size,
{
    fn render(&self, mut target: (&mut RenderDestination, RenderConstraints)) -> VuiResult<()> {
        if self.hover {
            let size = self.size();
            let surf = Surface::new(size.w, size.h, PixelFormatEnum::ARGB8888)?;
            let mut canvas = surf.into_canvas()?;
            canvas.set_draw_color(Color::RGB(0, 100, 0));
            canvas.clear();
            let surf = canvas.into_surface();
            target.blit_surface(&surf)?;
        }

        self.inner.render(target)
    }
}
