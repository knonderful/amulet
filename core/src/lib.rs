use std::fmt::{Debug, Display, Formatter};

pub mod component;
pub mod resource_manager;
pub mod font_manager;
pub mod math;
pub mod render;
pub mod util;
pub mod generator;

pub type VuiResult<T> = Result<T, VuiError>;
pub struct VuiError;

impl Debug for VuiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderError")
    }
}

impl Display for VuiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to render")
    }
}

impl std::error::Error for VuiError {}

impl From<sdl2::ttf::FontError> for VuiError {
    fn from(_: sdl2::ttf::FontError) -> Self {
        VuiError
    }
}

impl From<sdl2::render::TextureValueError> for VuiError {
    fn from(_: sdl2::render::TextureValueError) -> Self {
        VuiError
    }
}

impl From<String> for VuiError {
    fn from(_: String) -> Self {
        VuiError
    }
}

