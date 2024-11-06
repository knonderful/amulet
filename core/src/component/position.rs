use crate::component::{ComponentEvent, HandleEvent, Render, RenderConstraints};
use crate::geom::{Clip, Point};
use crate::VuiResult;

#[derive(Debug, Clone, Default)]
pub struct Position<C> {
    value: Point,
    inner: C,
}

impl<C> Position<C> {
    pub fn new(pos: Point, inner: C) -> Self {
        Self { value: pos, inner }
    }

    pub fn position(&self) -> Point {
        self.value
    }
}

impl<C> HandleEvent for Position<C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.inner
            .handle_event(state, event.clip(self.value.to_vector()))
    }
}

impl<C, R> Render<R> for Position<C>
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
        let Some(constraints) = constraints.clip(self.value.to_vector()) else {
            return Ok(());
        };
        self.inner.render(state, constraints, render_ctx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::component::component_check;
    use crate::component::noop::Noop;

    // Static check that we have all component traits implemented
    const _: () = component_check::<Position<Noop>, ()>();
}
