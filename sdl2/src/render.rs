use crate::lossy::LossyInto;
use amulet_core::component::RenderConstraints;
use sdl2::render::{Canvas, RenderTarget, WindowCanvas};
use sdl2::video::Window;
use std::ops::DerefMut;

pub trait SdlRender {
    type Target: RenderTarget;

    fn get_canvas(&mut self, constraints: RenderConstraints) -> &mut Canvas<Self::Target>;
}

impl<R> SdlRender for &mut R
where
    R: SdlRender,
{
    type Target = R::Target;

    fn get_canvas(&mut self, constraints: RenderConstraints) -> &mut Canvas<Self::Target> {
        self.deref_mut().get_canvas(constraints)
    }
}

pub struct RenderContext<'a> {
    canvas: &'a mut WindowCanvas,
}

impl<'a> RenderContext<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Self {
        Self { canvas }
    }
}

impl SdlRender for RenderContext<'_> {
    type Target = Window;

    fn get_canvas(&mut self, constraints: RenderConstraints) -> &mut Canvas<Self::Target> {
        let rect = constraints.clip_rect();
        let (x, y, w, h) = rect.into();
        self.canvas
            .set_viewport(sdl2::rect::Rect::new(x, y, w.lossy_into(), h.lossy_into()));
        self.canvas
    }
}
