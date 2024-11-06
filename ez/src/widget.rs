mod button;
mod factory;

use std::rc::Rc;
use crate::FramedTexture;
use amulet_core::component::{
    ComponentEvent, Frame, HandleEvent, Position, Render, RenderConstraints, Stack,
};
use amulet_core::VuiResult;
use amulet_sdl2::render::SdlRender;
pub use button::{Button, ButtonState};
pub use factory::WidgetFactory;
use sdl2::render::Texture;

struct Image<'a> {
    texture: Rc<Texture<'a>>,
}

impl<'a> Image<'a> {
    pub fn new(texture: Rc<Texture<'a>>) -> Self {
        Self { texture }
    }
}

impl HandleEvent for Image<'_> {
    // TODO: I guess there's no reason this should not be stackable
    type State<'a> = ();

    fn handle_event(&self, _: Self::State<'_>, _: ComponentEvent) -> VuiResult<()> {
        Ok(())
    }
}

impl<R> Render<R> for Image<'_>
where
    R: SdlRender,
{
    type State<'a> = ();

    fn render(
        &self,
        _: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        let canvas = render_ctx.get_canvas(constraints);
        canvas.copy(&self.texture, None, None)?;
        Ok(())
    }
}

trait IntoStack {
    type Output;

    fn into_stack(self) -> Self::Output;
}

impl<'a> IntoStack for FramedTexture<'a> {
    type Output = (Position, (Frame, Image<'a>));

    fn into_stack(self) -> Self::Output {
        let FramedTexture { rect, texture } = self;
        Image::new(texture)
            .stack(Frame::new(rect.size().cast()))
            .stack(Position::new(rect.min))
    }
}
