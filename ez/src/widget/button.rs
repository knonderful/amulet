use crate::widget::{Image, IntoStack};
use crate::FramedTexture;
use amulet_core::component::{
    ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position, Render,
    RenderConstraints, Stack,
};
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::SdlRender;

pub struct TupleEz<T, U>(T, U);

pub trait StackEz: Sized {
    fn stack_ez<N>(self, next: N) -> TupleEz<N, Self>;
}

impl<T> StackEz for T
    where
        T: Sized,
{
    fn stack_ez<N>(self, next: N) -> TupleEz<N, Self> {
        TupleEz(next, self)
    }
}

struct ButtonRender<'a> {
    unclicked: (Position, (Frame, Image<'a>)),
    clicked: (Position, (Frame, Image<'a>)),
}

impl<'a> ButtonRender<'a> {
    pub fn new(unclicked: FramedTexture<'a>, clicked: FramedTexture<'a>) -> Self {
        Self {
            unclicked: unclicked.into_stack(),
            clicked: clicked.into_stack(),
        }
    }
}


impl<C> HandleEvent for TupleEz<ButtonRender<'_>, C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let TupleEz(_, next) = &self;
        next.handle_event(state, event)
    }
}

impl<C, R> Render<R> for TupleEz<ButtonRender<'_>, C>
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
        let TupleEz(me, next) = &self;
        if ms_state.click_states().is_down(MouseButton::Left) {
            me.clicked.render((), constraints.clone(), render_ctx)?;
        } else {
            me.unclicked.render((), constraints.clone(), render_ctx)?;
        }

        next.render(inner_state, constraints, render_ctx)
    }
}

#[derive(Debug, Default)]
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

pub struct Button<'a> {
    #[allow(clippy::type_complexity)]
    #[rustfmt::skip]
    component: (Frame, (MouseSensor, TupleEz<ButtonRender<'a>, (Position, (Frame, Image<'a>))>), ),
}

impl<'a> Button<'a> {
    pub fn new(
        button_clicked: FramedTexture<'a>,
        button_unclicked: FramedTexture<'a>,
        label: FramedTexture<'a>,
    ) -> Self {
        let total_size = button_clicked
            .rect()
            .union(button_unclicked.rect())
            .max
            .to_u32()
            .to_vector()
            .into();

        let component = label
            .into_stack()
            .stack_ez(ButtonRender::new(button_unclicked, button_clicked))
            .stack(MouseSensor::new())
            .stack(Frame::new(total_size));

        Self { component }
    }
}

impl<R> Render<R> for Button<'_>
where
    R: SdlRender,
{
    type State<'a> = &'a ButtonState;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        let state = ().stack(&state.mouse_sensor);
        self.component.render(state, constraints, render_ctx)
    }
}

impl HandleEvent for Button<'_> {
    type State<'a> = &'a mut ButtonState;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.component
            .handle_event((&mut state.mouse_sensor, ()), event)
    }
}

#[cfg(test)]
mod test {
    use amulet_sdl2::render::RenderContext;

    use super::*;

    // Static check that we have all component traits implemented
    const _: () = amulet_core::component::component_check::<Button, &mut RenderContext>();
}
