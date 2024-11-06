use amulet_core::component::RenderConstraints;
use sdl2::render::{Canvas, RenderTarget, TextureCreator, WindowCanvas};
use sdl2::video::{Window, WindowContext};
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
    texture_creator: &'a TextureCreator<WindowContext>,
    canvas: &'a mut WindowCanvas,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &'a mut WindowCanvas,
    ) -> Self {
        Self {
            texture_creator,
            canvas,
        }
    }
}

impl SdlRender for RenderContext<'_> {
    type Target = Window;

    fn get_canvas(&mut self, constraints: RenderConstraints) -> &mut Canvas<Self::Target> {
        let rect = constraints.clip_rect();
        let origin = rect.min;
        let (w, h) = rect.size().cast().into();
        self.canvas
            .set_viewport(sdl2::rect::Rect::new(origin.x, origin.y, w, h));
        &mut self.canvas
    }
}
