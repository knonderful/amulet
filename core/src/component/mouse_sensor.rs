use crate::component::{ComponentEvent, HandleEvent, Render, RenderConstraints};
use crate::mouse::{ClickStates, HoverState};
use crate::VuiResult;

#[derive(Debug, Default, Clone)]
pub struct MouseSensorState {
    hover_state: HoverState,
    click_states: ClickStates,
}

impl MouseSensorState {
    pub fn clear_event_states(&mut self) {
        self.hover_state.clear_event_state();
        self.click_states.clear_event_state();
    }

    pub fn hover_state(&self) -> &HoverState {
        &self.hover_state
    }

    pub fn click_states(&self) -> &ClickStates {
        &self.click_states
    }
}

pub struct MouseSensor<C> {
    inner: C,
}

impl<C> MouseSensor<C> {
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C> HandleEvent for MouseSensor<C>
where
    C: HandleEvent,
{
    type State<'a> = (&'a mut MouseSensorState, C::State<'a>);

    fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        let (state, inner_state) = state;

        match &event {
            ComponentEvent::LoopStart => {
                state.clear_event_states();
            }
            ComponentEvent::MouseMotion(pos) => {
                state.hover_state.update(pos.is_hit());
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                if pos.is_hit() {
                    state.click_states.click(*btn);
                } else {
                    state.click_states.clear(*btn);
                }
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                if pos.is_hit() {
                    state.click_states.unclick(*btn);
                } else {
                    state.click_states.clear(*btn);
                }
            }
        }

        self.inner.handle_event(inner_state, event)
    }
}

impl<C, R> Render<R> for MouseSensor<C>
where
    C: Render<R>,
{
    type State<'a> = (&'a MouseSensorState, C::State<'a>);

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        self.inner.render(state.1, constraints, render_ctx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::component::component_check;
    use crate::component::noop::Noop;

    // Static check that we have all component traits implemented
    const _: () = component_check::<MouseSensor<Noop>, ()>();
}
