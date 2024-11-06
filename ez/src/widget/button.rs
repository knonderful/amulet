use crate::widget::Image;
use amulet_core::component::{
    AdjustLayout, AsChain, ComponentEvent, Frame, HandleEvent, Layout, MouseSensor,
    MouseSensorState, Position, SizeAttr,
};
use amulet_core::geom::Size;
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::{Render, RenderContext};

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
    outer: (Frame, MouseSensor),
    inner: (Position, Frame, Position),
    background: Image<'a>,
    content: Image<'a>,
}

impl SizeAttr for Button<'_> {
    fn size(&self) -> Size {
        self.outer.0.size()
    }
}

impl<'a> Button<'a> {
    pub fn new(
        outer: (Frame, MouseSensor),
        inner: (Position, Frame, Position),
        background: Image<'a>,
        content: Image<'a>,
    ) -> Self {
        Self {
            outer,
            inner,
            background,
            content,
        }
    }
}

impl HandleEvent for Button<'_> {
    type State<'a> = &'a mut ButtonState;

    fn handle_event(
        &self,
        state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        (self.outer.as_chain(), self.inner.as_chain())
            .as_chain()
            .handle_event((((), &mut state.mouse_sensor), ((), (), ())), event)
    }
}

impl Render for Button<'_> {
    type State<'a> = &'a ButtonState;

    fn render(
        &self,
        _state: Self::State<'_>,
        layout: Layout,
        render_context: &mut RenderContext,
    ) -> VuiResult<()> {
        let layout = self.outer.as_chain().adjust_layout(((), ()), layout)?;
        self.background.render((), layout.clone(), render_context)?;
        let layout = self.inner.as_chain().adjust_layout(((), (), ()), layout)?;
        self.content.render((), layout, render_context)
    }
}
