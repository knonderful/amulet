mod button;
mod factory;

use crate::FramedTexture;
use amulet_core::component::{Frame, HandleEvent, Position, Render, RenderConstraints};
use amulet_core::VuiResult;
use amulet_sdl2::lossy::LossyInto;
use amulet_sdl2::render::SdlRender;
pub use button::{Button, ButtonState};
pub use factory::WidgetFactory;
use sdl2::render::Texture;
use std::rc::Rc;

struct Image<'a> {
    texture: Rc<Texture<'a>>,
}

impl<'a> Image<'a> {
    pub fn new(texture: Rc<Texture<'a>>) -> Self {
        Self { texture }
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
            let size: (i32, i32) = constraints.clip_rect().size.into();
            let (w, h) = size.lossy_into();
            sdl2::rect::Rect::new(0, 0, w, h)
        };

        let canvas = render_ctx.get_canvas(constraints.clone());
        canvas.copy(&self.texture, rect, rect)?;
        Ok(constraints)
    }
}

trait IntoStack {
    type Output;

    fn into_stack(self) -> Self::Output;
}

impl<'a> IntoStack for FramedTexture<'a> {
    type Output = (Position, Frame, Image<'a>);

    fn into_stack(self) -> Self::Output {
        let FramedTexture { rect, texture } = self;
        (
            Position::new(rect.origin),
            Frame::new(rect.size),
            Image::new(texture),
        )
    }
}
