use amulet_core::component::{
    ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position, Render,
    RenderConstraints,
};
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::SdlRender;
use sdl2::surface::Surface;
use std::rc::Rc;

pub struct Prerendered {
    surface: Surface<'static>,
}

impl Prerendered {
    pub fn new(surface: Surface<'static>) -> Self {
        Self { surface }
    }
}

impl HandleEvent for Prerendered {
    type State<'a> = ();

    fn handle_event(&self, _: Self::State<'_>, _: ComponentEvent) -> VuiResult<()> {
        Ok(())
    }
}

impl<R> Render<R> for Prerendered
where
    R: SdlRender,
{
    type State<'a> = ();

    fn render(
        &self,
        _: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        render_ctx.blit_surface(constraints, &self.surface)
    }
}

pub type Label = Prerendered;

struct ButtonRender<C> {
    unclicked: Prerendered,
    clicked: Prerendered,
    inner: C,
}

impl<C> ButtonRender<C> {
    pub fn new(unclicked: Surface<'static>, clicked: Surface<'static>, inner: C) -> Self {
        Self {
            unclicked: Prerendered::new(unclicked),
            clicked: Prerendered::new(clicked),
            inner,
        }
    }
}

impl<C> HandleEvent for ButtonRender<C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.inner.handle_event(state, event)
    }
}

impl<C, R> Render<R> for ButtonRender<C>
where
    C: Render<R>,
    R: SdlRender,
{
    type State<'a> = (&'a MouseSensorState, C::State<'a>);

    fn render(
        &self,
        (ms_state, inner_state): Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        if ms_state.click_states().is_down(MouseButton::Left) {
            self.clicked.render((), constraints.clone(), render_ctx)?;
        } else {
            self.unclicked.render((), constraints.clone(), render_ctx)?;
        }

        self.inner.render(inner_state, constraints, render_ctx)
    }
}

pub struct ButtonState {
    mouse_sensor: MouseSensorState,
}

impl ButtonState {
    pub fn was_clicked(&self) -> bool {
        self.mouse_sensor
            .click_states()
            .has_click_completed(MouseButton::Left)
    }
}

pub struct Button {
    component: Frame<MouseSensor<ButtonRender<Position<Label>>>>,
}

impl Button {
    pub fn new(
        button_clicked: Surface<'static>,
        button_unclicked: Surface<'static>,
        label: Surface<'static>,
    ) -> Self {
        let component = Frame::new(
            button_clicked.size().into(),
            MouseSensor::new(ButtonRender::new(
                button_unclicked,
                button_clicked,
                Position::new((2, 2).into(), Label::new(label)),
            )),
        );
        Self { component }
    }
}

impl<R> Render<R> for Button
where
    R: SdlRender,
{
    type State<'a> = &'a mut ButtonState;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        // TODO: lol... MouseSensor technically doesn't need its own state in for Render...
        let state = (&state.mouse_sensor, (&state.mouse_sensor, ()));
        self.component.render(state, constraints, render_ctx)
    }
}

impl HandleEvent for Button {
    type State<'a> = &'a mut ButtonState;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.component
            .handle_event((&mut state.mouse_sensor, ()), event)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use amulet_sdl2::render::RenderContext;

    // Static check that we have all component traits implemented
    const _: () = amulet_core::component::component_check::<Button, &mut RenderContext>();
}
