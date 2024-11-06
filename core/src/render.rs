use crate::geom::{ComponentSize, Rect, Vector};
use crate::VuiResult;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

pub struct RenderDestination<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    canvas: &'a mut WindowCanvas,
}

impl<'a> RenderDestination<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &'a mut WindowCanvas,
    ) -> Self {
        RenderDestination {
            texture_creator,
            canvas,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderConstraints {
    clip_rect: Rect,
}

impl RenderConstraints {
    pub fn new(clip_rect: Rect) -> Self {
        Self { clip_rect }
    }

    pub fn clip(&self, rect: Rect) -> Option<Self> {
        self.clip_rect.intersection(&rect).map(Self::new)
    }

    pub fn clip_topleft(&self, delta: Vector) -> Option<Self> {
        self.clip(self.clip_rect.translate(delta))
    }

    pub fn clip_size(&self, size: ComponentSize) -> Option<Self> {
        let mut rect = self.clip_rect.clone();
        rect.set_size(size.to_i32());
        self.clip(rect)
    }
}

pub trait BlitSurface {
    fn blit_surface(&mut self, surface: &Surface) -> VuiResult<()>;
}

impl BlitSurface for (&mut RenderDestination<'_>, RenderConstraints) {
    fn blit_surface(&mut self, surface: &Surface) -> VuiResult<()> {
        let (dest, constraints) = self;
        let texture = dest.texture_creator.create_texture_from_surface(surface)?;
        let (x, y) = {
            let rect = constraints.clip_rect;
            (rect.min.x, rect.min.y)
        };

        let (w, h) = surface.size();
        // TODO: Clipping when the clip_rect is smaller than the surface... can either use set_clip_rect() or do it manually...
        //       Also note that for textures we wouldn't know the size... how do we handle that and is that relevant to this here...?
        dest.canvas.copy(&texture, None, SdlRect::new(x, y, w, h))?;
        Ok(())
    }
}
