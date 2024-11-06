use crate::theme::Theme;
use crate::widget::Button;
use crate::FramedTexture;
use amulet_core::geom::{Rect, Size};
use amulet_core::VuiResult;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

#[allow(unused)]
mod dev {
    //! Temporary module for dev tools.

    use sdl2::surface::Surface;

    pub fn outline(surf: Surface<'_>) -> Surface<'_> {
        let rect = surf.rect();
        let mut canv = surf.into_canvas().unwrap();
        canv.set_draw_color(sdl2::pixels::Color::RED);
        canv.draw_rect(rect).unwrap();
        canv.into_surface()
    }
}

pub struct WidgetFactory<'a> {
    theme: &'a Theme<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
}

const PADDING_H: i32 = 5;
const PADDING_V: i32 = 3;

impl<'a> WidgetFactory<'a> {
    pub fn new(theme: &'a Theme<'a>, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            theme,
            texture_creator,
        }
    }

    pub fn button(&mut self, text: &str) -> VuiResult<Button<'a>> {
        self.create_button(text, None)
    }

    pub fn button_set(&mut self, texts: &[&str]) -> VuiResult<Vec<Button<'a>>> {
        let mut max_size = Size::zero();
        for text in texts {
            max_size = max_size.max(self.theme.label_size(text)?);
        }

        let mut out = Vec::with_capacity(texts.len());
        for label in texts {
            out.push(self.create_button(label, Some(max_size))?);
        }

        Ok(out)
    }
}

// private stuff
impl<'a> WidgetFactory<'a> {
    fn create_button(&self, text: &str, label_size: Option<Size>) -> VuiResult<Button<'a>> {
        let label_surf = self.theme.label(text)?;
        let label_size = label_size.unwrap_or_else(|| Size::from(label_surf.size()));
        let diff_size = label_size - label_surf.size().into();
        let (x, y): (i32, i32) = diff_size.cast().into();
        let label_rect = Rect::from_size(Size::from(label_surf.size()).cast())
            .translate((PADDING_H, PADDING_V).into())
            .translate((x / 2, y / 2).into());
        let label = FramedTexture::new(label_rect, label_surf.as_texture(self.texture_creator)?);

        let button_rect = {
            let size = label_size + Size::new(PADDING_H as u32 * 2, PADDING_V as u32 * 2);
            Rect::from_size(size.cast())
        };

        let button_surf = self.theme.button(button_rect.size().cast())?;
        let background =
            FramedTexture::new(button_rect, button_surf.as_texture(self.texture_creator)?);

        Ok(Button::new(background, label))
    }
}
