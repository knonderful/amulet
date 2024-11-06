use crate::component::{AdjustLayout, ComponentEvent, HandleEvent, Layout, SizeAttr};
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
}

impl SizeAttr for Frame {
    fn size(&self) -> Size {
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

impl AdjustLayout for Frame {
    type State<'a> = ();

    fn adjust_layout(&self, _state: Self::State<'_>, layout: Layout) -> VuiResult<Layout> {
        Ok(layout.resize_clipped(self.size))
    }
}
