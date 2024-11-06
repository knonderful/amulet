use crate::component::{CalculateSize, ComponentEvent, HandleEvent, Render, RenderConstraints};
use crate::geom::Size;
use crate::VuiResult;

pub struct Noop;

impl HandleEvent for Noop {
    type State<'a> = ();

    fn handle_event(&self, _state: Self::State<'_>, _event: ComponentEvent) -> VuiResult<()> {
        Ok(())
    }
}

impl CalculateSize for Noop {
    fn calculate_size(&self) -> Size {
        Size::zero()
    }
}

impl<R> Render<R> for Noop {
    type State<'a> = ();

    fn render(&self, _: Self::State<'_>, _: RenderConstraints, _: R) -> VuiResult<()> {
        Ok(())
    }
}
