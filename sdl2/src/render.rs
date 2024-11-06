use amulet_core::component::RenderConstraints;
use amulet_core::VuiResult;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

pub trait SdlRender {
    fn blit_surface(
        &mut self,
        constraints: RenderConstraints,
        surface: &Surface,
    ) -> VuiResult<()>;
}

impl<T> SdlRender for &mut T where T: SdlRender {
    fn blit_surface(&mut self, constraints: RenderConstraints, surface: &Surface) -> VuiResult<()> {
        (**self).blit_surface(constraints, surface)
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
    fn blit_surface(
        &mut self,
        constraints: RenderConstraints,
        surface: &Surface,
    ) -> VuiResult<()> {
        let texture = self
            .texture_creator
            .create_texture_from_surface(surface)?;
        let (x, y) = {
            let rect = constraints.clip_rect();
            (rect.min.x, rect.min.y)
        };

        let (w, h) = surface.size();
        // TODO: Clipping when the clip_rect is smaller than the surface... can either use set_clip_rect() or do it manually...
        //       Also note that for textures we wouldn't know the size... how do we handle that and is that relevant to this here...?
        self.canvas.copy(&texture, None, SdlRect::new(x, y, w, h))?;
        Ok(())
    }
}
