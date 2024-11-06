use crate::component::{ComponentEvent, HandleEvent, Inner, InnerMut, MouseButton, Render, Size};
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
pub struct MouseAwareState {
    mouse_state: MouseState,
}

impl MouseAwareState {
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

pub struct MouseAware<C> {
    inner: C,
}

impl<C> MouseAware<C> {
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}
impl<C> MouseAware<C>
where
    C: Size,
{
    fn is_inside(&self, pos: Point) -> bool {
        let size = self.size();
        let rect = Rect::new(0, 0, size.w, size.h);
        rect.contains_point(pos)
    }
}

impl<C> Size for MouseAware<C>
where
    C: Size,
{
    fn size(&self) -> crate::math::Size {
        self.inner.size()
    }
}

impl<C> HandleEvent for MouseAware<C>
where
    C: HandleEvent + Size,
{
    type State<'a> = (&'a mut MouseAwareState, C::State<'a>);

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let (state, inner_state) = state;

        match event {
            ComponentEvent::LoopStart => {
                // Advance click states, such that we don't get "stuck" in intermediate states
                for btn in MouseButton::iter() {
                    match state.mouse_state.click_states.get(btn) {
                        ClickState::Neutral => {}
                        ClickState::Started => {
                            state.mouse_state.click_states.set(btn, ClickState::Active)
                        }
                        ClickState::Active => {}
                        ClickState::Completed => {
                            state.mouse_state.click_states.set(btn, ClickState::Neutral)
                        }
                    }
                }
            }
            ComponentEvent::MouseMotion(pos) => {
                let new_state = if self.is_inside(pos) {
                    HoverState::Inside
                } else {
                    HoverState::Outside
                };
                state.mouse_state.hover_state = new_state;
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                let new_state = if self.is_inside(pos) {
                    ClickState::Started
                } else {
                    ClickState::Neutral
                };
                state.mouse_state.click_states.set(btn, new_state);
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                let new_state = match state.mouse_state.click_states.get(btn) {
                    ClickState::Started | ClickState::Active => {
                        if self.is_inside(pos) {
                            ClickState::Completed
                        } else {
                            ClickState::Neutral
                        }
                    }
                    ClickState::Neutral | ClickState::Completed => ClickState::Neutral,
                };
                state.mouse_state.click_states.set(btn, new_state);
            }
        }

        self.inner.handle_event(inner_state, event)
    }
}

impl<C> Render for MouseAware<C>
where
    C: Render + Size,
{
    type State<'a> = (&'a MouseAwareState, C::State<'a>);

    fn render(
        &self,
        state: Self::State<'_>,
        mut target: (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let (state, inner_state) = state;
        if state.mouse_state.hover_state == HoverState::Inside {
            let size = self.size();
            let surf = Surface::new(size.w, size.h, PixelFormatEnum::ARGB8888)?;
            let mut canvas = surf.into_canvas()?;
            canvas.set_draw_color(Color::RGB(0, 100, 0));
            canvas.clear();
            let surf = canvas.into_surface();
            target.blit_surface(&surf)?;
        }

        self.inner.render(inner_state, target)
    }
}

impl<C> Inner for MouseAware<C> {
    type Component = C;

    fn inner(&self) -> &Self::Component {
        &self.inner
    }
}

impl<C> InnerMut for MouseAware<C> {
    type Component = C;

    fn inner_mut(&mut self) -> &mut Self::Component {
        &mut self.inner
    }
}
