use crate::component::{ComponentEvent, HandleEvent, Inner, InnerMut, Render, Size};
use crate::geom::{ComponentSize, Point, Rect};
use crate::math::LossyInto;
use crate::mouse::{ClickStates, HoverState};
use crate::render::{BlitSurface, RenderConstraints, RenderDestination};
use crate::VuiResult;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;

#[derive(Debug, Default, Clone)]
pub struct MouseSensorState {
    hover_state: HoverState,
    click_states: ClickStates,
}

impl MouseSensorState {
    pub fn clear_event_states(&mut self) {
        self.hover_state.clear_event_state();
        self.click_states.clear_event_state();
    }

    pub fn hover_state(&self) -> &HoverState {
        &self.hover_state
    }

    pub fn click_states(&self) -> &ClickStates {
        &self.click_states
    }
}

pub struct MouseSensor<C> {
    inner: C,
}

impl<C> MouseSensor<C> {
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}
impl<C> MouseSensor<C>
where
    C: Size,
{
    fn is_inside(&self, pos: Point) -> bool {
        let size = self.size();
        let rect = Rect::new(
            Point::origin(),
            Point::new(size.width.lossy_into(), size.height.lossy_into()),
        );
        rect.contains(pos)
    }
}

impl<C> Size for MouseSensor<C>
where
    C: Size,
{
    fn size(&self) -> ComponentSize {
        self.inner.size()
    }
}

impl<C> HandleEvent for MouseSensor<C>
where
    C: HandleEvent + Size,
{
    type State<'a> = (&'a mut MouseSensorState, C::State<'a>);

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let (state, inner_state) = state;

        match event {
            ComponentEvent::LoopStart => {
                state.clear_event_states();
            }
            ComponentEvent::MouseMotion(pos) => {
                state.hover_state.update(self.is_inside(pos));
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

impl<C> Render for MouseSensor<C>
where
    C: Render + Size,
{
    type State<'a> = (&'a MouseSensorState, C::State<'a>);

    fn render(
        &self,
        state: Self::State<'_>,
        mut target: (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        let (state, inner_state) = state;
        if state.hover_state.is_hovering() {
            let size = self.size();
            let surf = Surface::new(
                size.width.into(),
                size.height.into(),
                PixelFormatEnum::ARGB8888,
            )?;
            let mut canvas = surf.into_canvas()?;
            canvas.set_draw_color(Color::RGB(0, 100, 0));
            canvas.clear();
            let surf = canvas.into_surface();
            target.blit_surface(&surf)?;
        }

        self.inner.render(inner_state, target)
    }
}

impl<C> Inner for MouseSensor<C> {
    type Component = C;

    fn inner(&self) -> &Self::Component {
        &self.inner
    }
}

impl<C> InnerMut for MouseSensor<C> {
    type Component = C;

    fn inner_mut(&mut self) -> &mut Self::Component {
        &mut self.inner
    }
}
