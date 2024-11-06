use crate::widget::{Image, IntoStack};
use crate::FramedTexture;
use amulet_core::component::{ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position, Render, RenderConstraints, SizeAttr};
use amulet_core::geom::Size;
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::SdlRender;

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
    component: (Frame, MouseSensor, Image<'a>, Position, Frame, Image<'a>),
}

impl<'a> Button<'a> {
    pub fn new(background: FramedTexture<'a>, label: FramedTexture<'a>) -> Self {
        let total_size = background.rect.limit().as_size();
        let (lbl_pos, lbl_frame, lbl_img) = label.into_stack();
        let component = (
            Frame::new(total_size),
            MouseSensor::new(),
            Image::new(background.texture),
            lbl_pos,
            lbl_frame,
            lbl_img,
        );

        Self { component }
    }
}

impl SizeAttr for Button<'_> {
    fn size(&self) -> Size {
        self.component.0.size()
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
        self.component.handle_event(state, event.clone())
    }
}

impl<R> Render<R> for Button<'_>
where
    R: SdlRender,
{
    type State<'a> = &'a ButtonState;

    fn render(
        &self,
        _state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        let state = ((), (), (), (), (), ());
        self.component.render(state, constraints, render_ctx)
    }
}
