use crate::component::{ComponentEvent, HandleEvent, Inner, InnerMut, Render, Size};
use crate::geom::{ComponentSize, Point};
use crate::render::{RenderConstraints, RenderDestination};
use crate::VuiResult;

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

impl<C> Inner for Position<C> {
    type Component = C;

    fn inner(&self) -> &Self::Component {
        &self.inner
    }
}

impl<C> InnerMut for Position<C> {
    type Component = C;

    fn inner_mut(&mut self) -> &mut Self::Component {
        &mut self.inner
    }
}

impl<C> HandleEvent for Position<C>
where
    C: HandleEvent,
{
    type State<'a> = C::State<'a>;

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let event = match event {
            ComponentEvent::MouseMotion(pos) => {
                ComponentEvent::MouseMotion((pos - self.value).to_point())
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, (pos - self.value).to_point())
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, (pos - self.value).to_point())
            }
            other => other,
        };

        self.inner.handle_event(state, event)
    }
}

impl<C> Size for Position<C>
where
    C: Size,
{
    fn size(&self) -> ComponentSize {
        self.inner.size()
    }
}

impl<C, X> Render<X> for Position<C>
where
    C: Render<X>,
{
    type State<'a> = C::State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        (dest, constraints): (&mut RenderDestination, RenderConstraints),
        render_ctx: X,
    ) -> VuiResult<()> {
        let Some(constraints) = constraints.clip_topleft(self.value.to_vector()) else {
            return Ok(());
        };
        self.inner.render(state, (dest, constraints), render_ctx)
    }
}
