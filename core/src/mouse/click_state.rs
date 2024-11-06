use crate::bitops::{ClearBits, IsSet, SetBits};
use crate::mouse::MouseButton;
use std::fmt::{Debug, Formatter};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct ClickStates {
    click_state: u8,
    event_state: u8,
}

impl Debug for ClickStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}/{:08b}", self.click_state, self.event_state)
    }
}

fn mask_for(btn: MouseButton) -> u8 {
    1 << btn as u8
}

impl ClickStates {
    pub fn clear_event_state(&mut self) {
        self.event_state = 0;
    }

    pub fn has_click_started(&self, btn: MouseButton) -> bool {
        let mask = mask_for(btn);
        self.click_state.is_set(mask) && self.event_state.is_set(mask)
    }

    pub fn has_click_completed(&self, btn: MouseButton) -> bool {
        let mask = mask_for(btn);
        !self.click_state.is_set(mask) && self.event_state.is_set(mask)
    }

    pub fn is_up(&self, btn: MouseButton) -> bool {
        !self.click_state.is_set(mask_for(btn))
    }

    pub fn is_down(&self, btn: MouseButton) -> bool {
        self.click_state.is_set(mask_for(btn))
    }

    pub fn click(&mut self, btn: MouseButton) {
        let mask = mask_for(btn);
        self.click_state |= mask;
        self.event_state |= mask;
    }

    pub fn unclick(&mut self, btn: MouseButton) {
        let mask = mask_for(btn);

        if self.click_state.is_set(mask) {
            self.event_state.set_bits(mask);
        } else {
            self.event_state.clear_bits(mask);
        }

        self.click_state.clear_bits(mask);
    }

    pub fn clear(&mut self, btn: MouseButton) {
        let mask = mask_for(btn);
        self.click_state.clear_bits(mask);
        self.event_state.clear_bits(mask);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mouse_click_states() {
        trait CheckFlags {
            fn check_flags(&self, click_state: u8, update_state: u8);
        }

        impl CheckFlags for ClickStates {
            fn check_flags(&self, click_state: u8, update_state: u8) {
                assert_eq!(click_state, self.click_state);
                assert_eq!(update_state, self.event_state);
            }
        }

        let mut mcs = ClickStates::default();
        mcs.check_flags(0b000, 0b000);

        mcs.click(MouseButton::Middle);
        mcs.check_flags(0b010, 0b010);
        assert!(mcs.has_click_started(MouseButton::Middle));
        assert!(mcs.is_down(MouseButton::Middle));

        mcs.unclick(MouseButton::Middle);
        mcs.check_flags(0b000, 0b010);
        assert!(mcs.has_click_completed(MouseButton::Middle));
        assert!(mcs.is_up(MouseButton::Middle));

        mcs.click(MouseButton::Right);
        mcs.check_flags(0b100, 0b110);
        assert!(mcs.has_click_completed(MouseButton::Middle));
        assert!(mcs.is_up(MouseButton::Middle));
        assert!(mcs.has_click_started(MouseButton::Right));
        assert!(mcs.is_down(MouseButton::Right));

        mcs.clear_event_state();
        mcs.check_flags(0b100, 0b000);
        assert!(!mcs.has_click_started(MouseButton::Middle));
        assert!(!mcs.has_click_completed(MouseButton::Middle));
        assert!(mcs.is_up(MouseButton::Middle));
        assert!(!mcs.has_click_started(MouseButton::Right));
        assert!(!mcs.has_click_completed(MouseButton::Right));
        assert!(mcs.is_down(MouseButton::Right));

        mcs.click(MouseButton::Left);
        mcs.check_flags(0b101, 0b001);
        assert!(mcs.has_click_started(MouseButton::Left));

        mcs.clear(MouseButton::Right);
        mcs.check_flags(0b001, 0b001);
        assert!(!mcs.has_click_completed(MouseButton::Right));

        mcs.unclick(MouseButton::Middle);
        // This is a special case: a mouse-up on a button that has not been "started" should not
        // result in a click-completed.
        assert!(!mcs.has_click_completed(MouseButton::Middle));
        mcs.check_flags(0b001, 0b001);
    }
}
