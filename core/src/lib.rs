use std::fmt::{Debug, Display, Formatter};

pub mod bitops;
pub mod component;
pub mod geom;
pub mod math;
pub mod mouse;
pub mod render;

pub type VuiResult<T> = Result<T, VuiError>;

#[derive(Debug)]
pub struct VuiError {
    message: String,
}

impl VuiError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for VuiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VuiError {}

impl From<sdl2::ttf::FontError> for VuiError {
    fn from(e: sdl2::ttf::FontError) -> Self {
        Self::new(format!("FontError: {}", e))
    }
}

impl From<sdl2::render::TextureValueError> for VuiError {
    fn from(e: sdl2::render::TextureValueError) -> Self {
        Self::new(format!("TextureValueError: {}", e))
    }
}

impl From<String> for VuiError {
    fn from(e: String) -> Self {
        Self::new(e)
    }
}
