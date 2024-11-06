use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl TryFrom<sdl2::mouse::MouseButton> for MouseButton {
    type Error = ();

    fn try_from(value: sdl2::mouse::MouseButton) -> Result<Self, Self::Error> {
        use sdl2::mouse::MouseButton as MB;
        let out = match value {
            MB::Unknown | MB::X1 | MB::X2 => return Err(()),
            MB::Left => MouseButton::Left,
            MB::Middle => MouseButton::Middle,
            MB::Right => MouseButton::Right,
        };
        Ok(out)
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
pub struct MouseClickStates {
    click_state: u8,
    event_state: u8,
}

impl Debug for MouseClickStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}/{:08b}", self.click_state, self.event_state)
    }
}

trait BitIsSet {
    fn is_set(&self) -> bool;
}

impl BitIsSet for (u8, u8) {
    fn is_set(&self) -> bool {
        let (value, mask) = self;
        value & mask != 0
    }
}

impl MouseClickStates {
    fn mask_for(btn: MouseButton) -> u8 {
        1 << btn as u8
    }

    fn inverse_mask(mask: u8) -> u8 {
        mask ^ 0b11111111
    }

    pub fn clear_event_state(&mut self) {
        self.event_state = 0;
    }

    pub fn is_click_started(&self, btn: MouseButton) -> bool {
        let mask = Self::mask_for(btn);
        (self.click_state, mask).is_set() && (self.event_state, mask).is_set()
    }

    pub fn is_click_completed(&self, btn: MouseButton) -> bool {
        let mask = Self::mask_for(btn);
        !(self.click_state, mask).is_set() && (self.event_state, mask).is_set()
    }

    pub fn is_up(&self, btn: MouseButton) -> bool {
        !(self.click_state, Self::mask_for(btn)).is_set()
    }

    pub fn is_down(&self, btn: MouseButton) -> bool {
        (self.click_state, Self::mask_for(btn)).is_set()
    }

    pub fn click(&mut self, btn: MouseButton) {
        let mask = Self::mask_for(btn);
        self.click_state |= mask;
        self.event_state |= mask;
    }

    pub fn unclick(&mut self, btn: MouseButton) {
        let mask = Self::mask_for(btn);
        let inverse_mask = Self::inverse_mask(mask);

        if (self.click_state, mask).is_set() {
            self.event_state |= mask;
        } else {
            self.event_state &= inverse_mask;
        }

        self.click_state &= inverse_mask;
    }

    pub fn clear(&mut self, btn: MouseButton) {
        let mask = Self::inverse_mask(Self::mask_for(btn));
        self.click_state &= mask;
        self.event_state &= mask;
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

        impl CheckFlags for MouseClickStates {
            fn check_flags(&self, click_state: u8, update_state: u8) {
                assert_eq!(click_state, self.click_state);
                assert_eq!(update_state, self.event_state);
            }
        }

        let mut mcs = MouseClickStates::default();
        mcs.check_flags(0b000, 0b000);

        mcs.click(MouseButton::Middle);
        mcs.check_flags(0b010, 0b010);
        assert!(mcs.is_click_started(MouseButton::Middle));
        assert!(mcs.is_down(MouseButton::Middle));

        mcs.unclick(MouseButton::Middle);
        mcs.check_flags(0b000, 0b010);
        assert!(mcs.is_click_completed(MouseButton::Middle));
        assert!(mcs.is_up(MouseButton::Middle));

        mcs.click(MouseButton::Right);
        mcs.check_flags(0b100, 0b110);
        assert!(mcs.is_click_completed(MouseButton::Middle));
        assert!(mcs.is_up(MouseButton::Middle));
        assert!(mcs.is_click_started(MouseButton::Right));
        assert!(mcs.is_down(MouseButton::Right));

        mcs.clear_event_state();
        mcs.check_flags(0b100, 0b000);
        assert!(!mcs.is_click_started(MouseButton::Middle));
        assert!(!mcs.is_click_completed(MouseButton::Middle));
        assert!(mcs.is_up(MouseButton::Middle));
        assert!(!mcs.is_click_started(MouseButton::Right));
        assert!(!mcs.is_click_completed(MouseButton::Right));
        assert!(mcs.is_down(MouseButton::Right));

        mcs.click(MouseButton::Left);
        mcs.check_flags(0b101, 0b001);
        assert!(mcs.is_click_started(MouseButton::Left));

        mcs.clear(MouseButton::Right);
        mcs.check_flags(0b001, 0b001);
        assert!(!mcs.is_click_completed(MouseButton::Right));

        mcs.unclick(MouseButton::Middle);
        // This is a special case: a mouse-up on a button that has not been "started" should not
        // result in a click-completed.
        assert!(!mcs.is_click_completed(MouseButton::Middle));
        mcs.check_flags(0b001, 0b001);
    }
}
