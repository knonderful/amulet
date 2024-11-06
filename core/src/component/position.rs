use crate::component::{ComponentEvent, HandleEvent, PositionAttr, Render, RenderConstraints};
use crate::geom::Point;
use crate::VuiResult;

#[derive(Debug, Clone, Default)]
pub struct Position {
    value: Point,
}

impl Position {
    pub fn new(pos: Point) -> Self {
        Self { value: pos }
    }
}

impl PositionAttr for Position {
    fn position(&self) -> Point {
        self.value
    }
}

impl HandleEvent for Position {
    type State<'a> = ();

    fn handle_event(
        &self,
        _state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        Ok(event.clip(self.value.as_vector()))
    }
}

impl<R> Render<R> for Position {
    type State<'a> = ();

    fn render(
        &self,
        _state: Self::State<'_>,
        constraints: RenderConstraints,
        _render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        Ok(constraints.clip(self.value.as_vector()))
    }
}
