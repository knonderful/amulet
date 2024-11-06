use crate::component::{CalculateSize, ComponentEvent, HandleEvent, Render, RenderConstraints};
use crate::geom::{Shrink, Size};
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

impl<C> HandleEvent for (Frame, C)
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let (me, next) = self;
        next.handle_event(state, event.shrink(me.size))
    }
}

impl<C, R> Render<R> for (Frame, C)
where
    C: Render<R>,
{
    type State<'a> = C::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        let (me, next) = self;
        let Some(constraints) = constraints.shrink(me.size) else {
            return Ok(());
        };
        next.render(state, constraints, render_ctx)
    }
}

impl CalculateSize for Frame {
    fn calculate_size(&self) -> Size {
        self.size
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::component::noop::Noop;
    use crate::component::component_check;

    // Static check that we have all component traits implemented
    const _: () = component_check::<(Frame, Noop), ()>();
}
