use crate::math::{Resized, Size, Translated};
use crate::VuiResult;
use sdl2::rect::{Point, Rect};
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

    pub fn size(&self) -> Size {
        self.canvas.window().size().into()
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
        self.clip_rect.intersection(rect).map(Self::new)
    }

    pub fn clip_topleft(&self, delta: Point) -> Option<Self> {
        self.clip(self.clip_rect.translated(delta))
    }

    pub fn clip_size(&self, size: Size) -> Option<Self> {
        self.clip(self.clip_rect.resized(size))
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
            (rect.x(), rect.y())
        };

        let (w, h) = surface.size();
        dest.canvas.copy(&texture, None, Rect::new(x, y, w, h))?;
        Ok(())
    }
}
