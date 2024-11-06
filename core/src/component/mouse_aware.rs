use crate::component::{ComponentEvent, HandleEvent, Inner, InnerMut, Render, Size};
use crate::mouse::MouseClickStates;
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;

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
pub struct MouseAwareState {
    hover_state: HoverState,
    click_states: MouseClickStates,
}

impl MouseAwareState {
    pub fn hovering(&self) -> bool {
        self.hover_state == HoverState::Inside
    }

    pub fn click_states(&self) -> &MouseClickStates {
        &self.click_states
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
                state.click_states.clear_event_state();
            }
            ComponentEvent::MouseMotion(pos) => {
                let new_state = if self.is_inside(pos) {
                    HoverState::Inside
                } else {
                    HoverState::Outside
                };
                state.hover_state = new_state;
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                if self.is_inside(pos) {
                    state.click_states.click(btn);
                } else {
                    state.click_states.clear(btn);
                }
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                if self.is_inside(pos) {
                    state.click_states.unclick(btn);
                } else {
                    state.click_states.clear(btn);
                }
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
        if state.hover_state == HoverState::Inside {
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
