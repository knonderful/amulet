use crate::component::{CalculateSize, ComponentEvent, HandleEvent, Render, RenderConstraints};
use crate::geom::{Shrink, Size};
use crate::VuiResult;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Frame<C> {
    size: Size,
    inner: C,
}

impl<C> Frame<C> {
    pub fn new(size: Size, inner: C) -> Self {
        Self { size, inner }
    }
}

impl<C> HandleEvent for Frame<C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.inner.handle_event(state, event.shrink(self.size))
    }
}

impl<C, R> Render<R> for Frame<C>
where
    C: Render<R>,
{
    type State<'a> = C::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: R,
    ) -> VuiResult<()> {
        self.inner
            .render(state, constraints.shrink(self.size), render_ctx)
    }
}

impl<C> CalculateSize for Frame<C> {
    fn calculate_size(&self) -> Size {
        self.size
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::component::noop::Noop;
    use crate::component::sized_component_check;

    // Static check that we have all component traits implemented
    const _: () = sized_component_check::<Frame<Noop>, ()>();
}
