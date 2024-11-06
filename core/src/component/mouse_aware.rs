use crate::component::{ComponentEvent, HandleEvent, MouseButton, Render, Size};
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;
use strum::{EnumCount as _, IntoEnumIterator as _};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClickState {
    Neutral,
    Started,
    Active,
    Completed,
}

impl Default for ClickState {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Debug, Default, Clone)]
struct ClickStateMap {
    states: [ClickState; MouseButton::COUNT],
}

impl ClickStateMap {
    fn get(&self, button: MouseButton) -> ClickState {
        self.states[button as usize]
    }

    fn set(&mut self, button: MouseButton, new_state: ClickState) {
        self.states[button as usize] = new_state
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HoverState {
    Undefined,
    Inside,
    Outside,
}

impl Default for HoverState {
    fn default() -> Self {
        Self::Undefined
    }
}

#[derive(Debug, Default, Clone)]
struct MouseState {
    hover_state: HoverState,
    click_states: ClickStateMap,
}

#[derive(Debug, Default, Clone)]
pub struct MouseAwareCtx {
    mouse_state: MouseState,
}

impl MouseAwareCtx {
    pub fn hovering(&self) -> bool {
        self.mouse_state.hover_state == HoverState::Inside
    }

    pub fn click_started(&self, button: MouseButton) -> bool {
        self.mouse_state.click_states.get(button) == ClickState::Started
    }

    pub fn click_completed(&self, button: MouseButton) -> bool {
        self.mouse_state.click_states.get(button) == ClickState::Completed
    }
}

pub struct MouseAware<'a, C> {
    ctx: &'a mut MouseAwareCtx,
    inner: C,
}

impl<'a, C> MouseAware<'a, C> {
    pub fn new(ctx: &'a mut MouseAwareCtx, inner: C) -> Self {
        // Advance click states, such that we don't get "stuck" in intermediate states
        for btn in MouseButton::iter() {
            match ctx.mouse_state.click_states.get(btn) {
                ClickState::Neutral => {}
                ClickState::Started => ctx.mouse_state.click_states.set(btn, ClickState::Active),
                ClickState::Active => {}
                ClickState::Completed => ctx.mouse_state.click_states.set(btn, ClickState::Neutral),
            }
        }

        Self { inner, ctx }
    }
}
impl<C> MouseAware<'_, C>
where
    C: Size,
{
    fn is_inside(&self, pos: Point) -> bool {
        let size = self.size();
        let rect = Rect::new(0, 0, size.w, size.h);
        rect.contains_point(pos)
    }
}

impl<C> Size for MouseAware<'_, C>
where
    C: Size,
{
    fn size(&self) -> crate::math::Size {
        self.inner.size()
    }
}

impl<C> HandleEvent for MouseAware<'_, C>
where
    C: HandleEvent + Size,
{
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        match event {
            ComponentEvent::MouseMotion(pos) => {
                let new_state = if self.is_inside(pos) {
                    HoverState::Inside
                } else {
                    HoverState::Outside
                };
                self.ctx.mouse_state.hover_state = new_state;
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                let new_state = if self.is_inside(pos) {
                    ClickState::Started
                } else {
                    ClickState::Neutral
                };
                self.ctx.mouse_state.click_states.set(btn, new_state);
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                let new_state = match self.ctx.mouse_state.click_states.get(btn) {
                    ClickState::Started | ClickState::Active => {
                        if self.is_inside(pos) {
                            ClickState::Completed
                        } else {
                            ClickState::Neutral
                        }
                    }
                    ClickState::Neutral | ClickState::Completed => ClickState::Neutral,
                };
                self.ctx.mouse_state.click_states.set(btn, new_state);
            }
        }
        self.inner.handle_event(event)
    }
}

impl<C> Render for MouseAware<'_, C>
where
    C: Render + Size,
{
    fn render(&self, mut target: (&mut RenderDestination, RenderConstraints)) -> VuiResult<()> {
        if self.ctx.mouse_state.hover_state == HoverState::Inside {
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
