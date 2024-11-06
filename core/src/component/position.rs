use crate::component::{ComponentEvent, HandleEvent, Layout, PositionAttr, UpdateLayout};
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

impl UpdateLayout for Position {
    type State<'a> = ();

    fn update_layout(&self, _state: Self::State<'_>, layout: Layout) -> VuiResult<Layout> {
        Ok(layout.clip(self.value.as_vector()))
    }
}
