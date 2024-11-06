use crate::component::{ComponentEvent, HandleEvent, Render, RenderConstraints};
use crate::geom::Size;
use crate::VuiResult;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Frame {
    size: Size,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl HandleEvent for Frame {
    type State<'a> = ();

    fn handle_event(
        &self,
        _state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        Ok(event.resize(self.size))
    }
}

impl<R> Render<R> for Frame {
    type State<'a> = ();

    fn render(
        &self,
        _state: Self::State<'_>,
        constraints: RenderConstraints,
        _render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        Ok(constraints.resize(self.size))
    }
}
