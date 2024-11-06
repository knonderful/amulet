use crate::component::{AdjustLayout, ComponentEvent, HandleEvent};
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

pub struct MouseSensor {}

impl MouseSensor {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandleEvent for MouseSensor {
    type State<'a> = &'a mut MouseSensorState;

    fn handle_event(
        &self,
        state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
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

        Ok(event)
    }
}

impl AdjustLayout for MouseSensor {
    type State<'a> = ();
}
