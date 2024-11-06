use crate::theme::Theme;
use amulet_core::component::Layout;
use amulet_core::VuiResult;
use amulet_sdl2::render::{Render, RenderContext};
use sdl2::pixels::Color;

pub struct DynText<'a> {
    theme: &'a Theme<'a>,
}

impl<'a> DynText<'a> {
    pub fn new(theme: &'a Theme<'a>) -> Self {
        Self { theme }
    }
}

impl Render for DynText<'_> {
    type State<'a> = &'a str;

    fn render(
        &self,
        text: Self::State<'_>,
        layout: Layout,
        render_context: &mut RenderContext,
    ) -> VuiResult<()> {
        if text.is_empty() {
            // SDL2 will actually panic if we try to render empty text
            return Ok(());
        }

        let surface = self.theme.font.render(text).blended(Color::GREEN)?;
        let (w, h) = surface.size();
        let texture = surface.as_texture(self.theme.texture_creator)?;
        render_context.get_canvas(layout).copy(
            &texture,
            None,
            Some(sdl2::rect::Rect::new(0, 0, w, h)),
        )?;
        Ok(())
    }
}
