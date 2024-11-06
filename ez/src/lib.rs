use amulet_core::geom::{Rect, Size};
use sdl2::render::Texture;
use std::rc::Rc;

pub mod theme;
pub mod widget;

#[derive(Clone)]
pub struct FramedTexture<'a> {
    rect: Rect,
    texture: Rc<Texture<'a>>,
}

impl<'a> FramedTexture<'a> {
    pub fn new(rect: Rect, texture: Texture<'a>) -> Self {
        Self {
            rect,
            texture: Rc::new(texture),
        }
    }
}

impl FramedTexture<'_> {
    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn size(&self) -> Size {
        self.rect.size().cast()
    }
}
