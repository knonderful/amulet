use amulet_core::component::{
    ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position, Render,
    RenderConstraints, Stack,
};
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::SdlRender;
use sdl2::surface::Surface;

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

struct ButtonRender {
    unclicked: Prerendered,
    clicked: Prerendered,
}

impl ButtonRender {
    pub fn new(unclicked: Surface<'static>, clicked: Surface<'static>) -> Self {
        Self {
            unclicked: Prerendered::new(unclicked),
            clicked: Prerendered::new(clicked),
        }
    }
}

pub struct EzTuple<T, U>(T, U);

pub trait EzStack: Sized {
    fn ez_stack<N>(self, next: N) -> EzTuple<N, Self>;
}

impl<T> EzStack for T
where
    T: Sized,
{
    fn ez_stack<N>(self, next: N) -> EzTuple<N, Self> {
        EzTuple(next, self)
    }
}

impl<C> HandleEvent for EzTuple<ButtonRender, C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let EzTuple(_, next) = &self;
        next.handle_event(state, event)
    }
}

impl<C, R> Render<R> for EzTuple<ButtonRender, C>
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
        let EzTuple(me, next) = &self;
        if ms_state.click_states().is_down(MouseButton::Left) {
            me.clicked.render((), constraints.clone(), render_ctx)?;
        } else {
            me.unclicked.render((), constraints.clone(), render_ctx)?;
        }

        next.render(inner_state, constraints, render_ctx)
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
    #[allow(clippy::type_complexity)]
    #[rustfmt::skip]
    component: (Frame, (MouseSensor, EzTuple<ButtonRender, (Position, Label)>), ),
}

impl Button {
    pub fn new(
        button_clicked: Surface<'static>,
        button_unclicked: Surface<'static>,
        label: Surface<'static>,
    ) -> Self {
        let frame_size = button_clicked.size().into();
        let component = Label::new(label)
            .stack(Position::new((2, 2).into()))
            .ez_stack(ButtonRender::new(button_unclicked, button_clicked))
            .stack(MouseSensor::new())
            .stack(Frame::new(frame_size));

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
