use crate::bitops::{ClearBits, IsSet, SetBits};
use crate::mouse::Button;
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

fn mask_for(btn: Button) -> u8 {
    1 << btn as u8
}

impl ClickStates {
    pub fn clear_event_state(&mut self) {
        self.event_state = 0;
    }

    pub fn has_click_started(&self, btn: Button) -> bool {
        let mask = mask_for(btn);
        self.click_state.is_set(mask) && self.event_state.is_set(mask)
    }

    pub fn has_click_completed(&self, btn: Button) -> bool {
        let mask = mask_for(btn);
        !self.click_state.is_set(mask) && self.event_state.is_set(mask)
    }

    pub fn is_up(&self, btn: Button) -> bool {
        !self.click_state.is_set(mask_for(btn))
    }

    pub fn is_down(&self, btn: Button) -> bool {
        self.click_state.is_set(mask_for(btn))
    }

    pub fn click(&mut self, btn: Button) {
        let mask = mask_for(btn);
        self.click_state |= mask;
        self.event_state |= mask;
    }

    pub fn unclick(&mut self, btn: Button) {
        let mask = mask_for(btn);

        if self.click_state.is_set(mask) {
            self.event_state.set_bits(mask);
        } else {
            self.event_state.clear_bits(mask);
        }

        self.click_state.clear_bits(mask);
    }

    pub fn clear(&mut self, btn: Button) {
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

        mcs.click(Button::Middle);
        mcs.check_flags(0b010, 0b010);
        assert!(mcs.has_click_started(Button::Middle));
        assert!(mcs.is_down(Button::Middle));

        mcs.unclick(Button::Middle);
        mcs.check_flags(0b000, 0b010);
        assert!(mcs.has_click_completed(Button::Middle));
        assert!(mcs.is_up(Button::Middle));

        mcs.click(Button::Right);
        mcs.check_flags(0b100, 0b110);
        assert!(mcs.has_click_completed(Button::Middle));
        assert!(mcs.is_up(Button::Middle));
        assert!(mcs.has_click_started(Button::Right));
        assert!(mcs.is_down(Button::Right));

        mcs.clear_event_state();
        mcs.check_flags(0b100, 0b000);
        assert!(!mcs.has_click_started(Button::Middle));
        assert!(!mcs.has_click_completed(Button::Middle));
        assert!(mcs.is_up(Button::Middle));
        assert!(!mcs.has_click_started(Button::Right));
        assert!(!mcs.has_click_completed(Button::Right));
        assert!(mcs.is_down(Button::Right));

        mcs.click(Button::Left);
        mcs.check_flags(0b101, 0b001);
        assert!(mcs.has_click_started(Button::Left));

        mcs.clear(Button::Right);
        mcs.check_flags(0b001, 0b001);
        assert!(!mcs.has_click_completed(Button::Right));

        mcs.unclick(Button::Middle);
        // This is a special case: a mouse-up on a button that has not been "started" should not
        // result in a click-completed.
        assert!(!mcs.has_click_completed(Button::Middle));
        mcs.check_flags(0b001, 0b001);
    }
}
