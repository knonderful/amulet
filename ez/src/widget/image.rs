use amulet_core::component::{Layout, SizeAttr};
use amulet_core::geom::Size;
use amulet_core::VuiResult;
use amulet_sdl2::lossy::LossyInto;
use amulet_sdl2::render::{Render, RenderContext};
use sdl2::render::Texture;
use std::rc::Rc;

#[derive(Clone)]
pub struct Image<'a> {
    texture: Rc<Texture<'a>>,
    size: Size,
}

impl<'a> Image<'a> {
    pub fn new(texture: Rc<Texture<'a>>, size: Size) -> Self {
        Self { texture, size }
    }
}

impl Render for Image<'_> {
    type State<'a> = ();

    fn render(
        &self,
        _state: Self::State<'_>,
        layout: Layout,
        render_ctx: &mut RenderContext,
    ) -> VuiResult<()> {
        let rect = {
            let size: (i32, i32) = self.size.into();
            let (w, h) = size.lossy_into();
            sdl2::rect::Rect::new(0, 0, w, h)
        };

        let canvas = render_ctx.get_canvas(layout);
        canvas.copy(&self.texture, rect, rect)?;

        Ok(())
    }
}

impl SizeAttr for Image<'_> {
    fn size(&self) -> Size {
        self.size
    }
}
