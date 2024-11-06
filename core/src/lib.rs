use crate::component::{ComponentEvent, FramedPosition};
use crate::geom::{Point, Rect};
use crate::mouse::Button;
use std::fmt::{Debug, Display, Formatter};

pub mod bitops;
pub mod component;
pub mod geom;
pub mod mouse;

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

impl From<String> for VuiError {
    fn from(e: String) -> Self {
        Self::new(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalEvent {
    LoopStart,
    MouseMotion(Point),
    MouseButtonDown(Button, Point),
    MouseButtonUp(Button, Point),
}

impl GlobalEvent {
    pub fn into_component_event(self, comp_rect: Rect) -> ComponentEvent {
        match self {
            GlobalEvent::LoopStart => ComponentEvent::LoopStart,
            GlobalEvent::MouseMotion(pos) => {
                ComponentEvent::MouseMotion(FramedPosition::new(pos, comp_rect))
            }
            GlobalEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, FramedPosition::new(pos, comp_rect))
            }
            GlobalEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, FramedPosition::new(pos, comp_rect))
            }
        }
    }
}
