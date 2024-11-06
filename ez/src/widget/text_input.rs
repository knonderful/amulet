use amulet_core::component::{
    AsChain, ComponentEvent, Frame, HandleEvent, Layout, MouseSensor, MouseSensorState, Position,
    SizeAttr, UpdateLayout,
};
use amulet_core::geom::Size;
use amulet_core::VuiResult;
use amulet_sdl2::render::{Render, RenderContext};
use crate::widget::{DynText, Image};

#[derive(Debug, Default)]
pub struct TextInputState {
    mouse_sensor: MouseSensorState,
    text: String,
}

impl TextInputState {
    pub fn update(&mut self, text: &str) {
        self.text.push_str(text)
    }
}

pub struct TextInput<'a> {
    outer: (Frame, MouseSensor),
    inner: (Position, Frame, Position),
    background: Image<'a>,
    content: DynText<'a>,
}

impl SizeAttr for TextInput<'_> {
    fn size(&self) -> Size {
        self.outer.0.size()
    }
}

impl<'a> TextInput<'a> {
    pub fn new(
        outer: (Frame, MouseSensor),
        inner: (Position, Frame, Position),
        background: Image<'a>,
        content: DynText<'a>,
    ) -> Self {
        Self {
            outer,
            inner,
            background,
            content,
        }
    }
}

impl HandleEvent for TextInput<'_> {
    type State<'a> = &'a mut TextInputState;

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

impl Render for TextInput<'_> {
    type State<'a> = &'a TextInputState;

    fn render(
        &self,
        state: Self::State<'_>,
        layout: Layout,
        render_context: &mut RenderContext,
    ) -> VuiResult<()> {
        let layout = self.outer.as_chain().update_layout(((), ()), layout)?;
        self.background.render((), layout.clone(), render_context)?;
        let layout = self.inner.as_chain().update_layout(((), (), ()), layout)?;
        self.content.render(&state.text, layout, render_context)
    }
}
