use crate::bitops::{ClearBits, IsSet, SetBits};

const HOVER_MASK: u8 = 0b00000001;
const EVENT_MASK: u8 = 0b00000010;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct HoverState {
    flags: u8,
}

impl HoverState {
    pub fn clear_event_state(&mut self) {
        self.flags.clear_bits(EVENT_MASK);
    }

    pub fn is_hovering(&self) -> bool {
        self.flags.is_set(HOVER_MASK)
    }

    pub fn has_entered(&self) -> bool {
        self.flags.is_set(HOVER_MASK | EVENT_MASK)
    }

    pub fn has_left(&self) -> bool {
        !self.flags.is_set(HOVER_MASK) && self.flags.is_set(EVENT_MASK)
    }

    pub fn update(&mut self, inside: bool) {
        if inside != self.is_hovering() {
            self.flags.set_bits(EVENT_MASK);
        }

        if inside {
            self.flags.set_bits(HOVER_MASK)
        } else {
            self.flags.clear_bits(HOVER_MASK)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hover_state() {
        let mut hs = HoverState::default();
        assert_eq!(0b00, hs.flags);

        hs.update(true);
        assert_eq!(0b11, hs.flags);
        assert!(hs.has_entered());
        assert!(hs.is_hovering());
        assert!(!hs.has_left());

        hs.clear_event_state();
        hs.update(true);
        assert_eq!(0b01, hs.flags);
        assert!(!hs.has_entered());
        assert!(hs.is_hovering());
        assert!(!hs.has_left());

        hs.clear_event_state();
        hs.update(true);
        assert_eq!(0b01, hs.flags);
        assert!(!hs.has_entered());
        assert!(hs.is_hovering());
        assert!(!hs.has_left());

        hs.clear_event_state();
        hs.update(false);
        assert_eq!(0b10, hs.flags);
        assert!(!hs.has_entered());
        assert!(!hs.is_hovering());
        assert!(hs.has_left());

        hs.clear_event_state();
        hs.update(false);
        assert_eq!(0b00, hs.flags);
        assert!(!hs.has_entered());
        assert!(!hs.is_hovering());
        assert!(!hs.has_left());
    }
}