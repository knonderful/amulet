#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Button {
    Left,
    Middle,
    Right,
}

impl TryFrom<sdl2::mouse::MouseButton> for Button {
    type Error = ();

    fn try_from(value: sdl2::mouse::MouseButton) -> Result<Self, Self::Error> {
        use sdl2::mouse::MouseButton as MB;
        let out = match value {
            MB::Unknown | MB::X1 | MB::X2 => return Err(()),
            MB::Left => Button::Left,
            MB::Middle => Button::Middle,
            MB::Right => Button::Right,
        };
        Ok(out)
    }
}
