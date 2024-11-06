use crate::widget::Image;
use amulet_core::component::{
    ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position, Render,
    RenderConstraints, SizeAttr,
};
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
    pub(crate) component: (Frame, MouseSensor, Image<'a>, Position, Frame),
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
        let state = ((), &mut state.mouse_sensor, (), (), ());
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
        let state = ((), (), (), (), ());
        self.component.render(state, constraints, render_ctx)
    }
}
