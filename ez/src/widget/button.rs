use crate::widget::{Image, IntoStack};
use crate::FramedTexture;
use amulet_core::component::{
    ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position, Render,
    RenderConstraints,
};
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::SdlRender;

struct ButtonRender<'a> {
    unclicked: (Position, Frame, Image<'a>),
    clicked: (Position, Frame, Image<'a>),
}

impl<'a> ButtonRender<'a> {
    pub fn new(unclicked: FramedTexture<'a>, clicked: FramedTexture<'a>) -> Self {
        Self {
            unclicked: unclicked.into_stack(),
            clicked: clicked.into_stack(),
        }
    }
}

impl HandleEvent for ButtonRender<'_> {
    type State<'a> = ();
}

impl<R> Render<R> for ButtonRender<'_>
where
    R: SdlRender,
{
    type State<'a> = &'a MouseSensorState;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        let state_stack = ((), (), ());
        if state.click_states().is_down(MouseButton::Left) {
            self.clicked
                .render(state_stack, constraints.clone(), render_ctx)?;
        } else {
            self.unclicked
                .render(state_stack, constraints.clone(), render_ctx)?;
        }
        Ok(constraints)
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
    component: (Frame, MouseSensor, ButtonRender<'a>, Position, Frame, Image<'a>),
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

        let (lbl_pos, lbl_frame, lbl_img) = label.into_stack();
        let component = (
            Frame::new(total_size),
            MouseSensor::new(),
            ButtonRender::new(button_unclicked, button_clicked),
            lbl_pos,
            lbl_frame,
            lbl_img,
        );

        Self { component }
    }
}

impl HandleEvent for Button<'_> {
    type State<'a> = &'a mut ButtonState;

    fn handle_event(
        &self,
        state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        let state = ((), &mut state.mouse_sensor, (), (), (), ());
        self.component.handle_event(state, event)
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
    ) -> VuiResult<RenderConstraints> {
        let state = ((), (), &state.mouse_sensor, (), (), ());
        self.component.render(state, constraints, render_ctx)
    }
}
