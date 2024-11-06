mod button;

use amulet_core::component::{HandleEvent, Render, RenderConstraints, SizeAttr};
use amulet_core::geom::Size;
use amulet_core::VuiResult;
use amulet_sdl2::lossy::LossyInto;
use amulet_sdl2::render::SdlRender;
pub use button::{Button, ButtonState};
use sdl2::render::Texture;
use std::rc::Rc;

pub struct Image<'a> {
    texture: Rc<Texture<'a>>,
    size: Size,
}

impl<'a> Image<'a> {
    pub fn new(texture: Rc<Texture<'a>>, size: Size) -> Self {
        Self { texture, size }
    }
}

impl HandleEvent for Image<'_> {
    type State<'a> = ();
}

impl<R> Render<R> for Image<'_>
where
    R: SdlRender,
{
    type State<'a> = ();

    fn render(
        &self,
        _state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        let rect = {
            let size: (i32, i32) = self.size.into();
            let (w, h) = size.lossy_into();
            sdl2::rect::Rect::new(0, 0, w, h)
        };

        let canvas = render_ctx.get_canvas(constraints.clone());
        canvas.copy(&self.texture, rect, rect)?;
        Ok(constraints)
    }
}

impl SizeAttr for Image<'_> {
    fn size(&self) -> Size {
        self.size
    }
}
