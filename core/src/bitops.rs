pub trait IsSet {
    fn is_set(&self, mask: Self) -> bool;
}

impl IsSet for u8 {
    fn is_set(&self, mask: Self) -> bool {
        self & mask == mask
    }
}

pub trait SetBits {
    fn set_bits(&mut self, mask: Self);
}

impl SetBits for u8 {
    fn set_bits(&mut self, mask: Self) {
        *self |= mask;
    }
}

pub trait ClearBits {
    fn clear_bits(&mut self, mask: Self);
}

impl ClearBits for u8 {
    fn clear_bits(&mut self, mask: Self) {
        *self &= mask ^ 0b11111111
    }
}
