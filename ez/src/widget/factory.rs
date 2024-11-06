use crate::theme::Theme;
use crate::widget::Button;
use crate::FramedTexture;
use amulet_core::geom::{Point, Rect, Size};
use amulet_core::VuiResult;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

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
        let label = self.theme.label(text)?;
        let label_size = Size::from(label.size());
        let (padding_h, padding_v) = (PADDING_H, PADDING_V);
        let label_rect =
            Rect::from_origin_and_size(Point::new(padding_h, padding_v), label_size.cast());
        let button_rect = label_rect.inflate(padding_h, padding_v);
        let button_surf = self.theme.button(button_rect.size().cast())?;

        let background =
            FramedTexture::new(button_rect, button_surf.as_texture(self.texture_creator)?);
        let label = FramedTexture::new(label_rect, label.as_texture(self.texture_creator)?);

        Ok(Button::new(background, label))
    }

    pub fn button_set(&mut self, texts: &[&str]) -> VuiResult<Vec<Button<'a>>> {
        let mut labels = Vec::with_capacity(texts.len());
        let mut max_size = Size::zero();
        for text in texts {
            let surface = self.theme.label(text)?;
            max_size = max_size.max(surface.size().into());
            labels.push(surface);
        }

        let (padding_h, padding_v) = (PADDING_H, PADDING_V);

        let mut out = Vec::with_capacity(labels.len());
        for label in labels {
            let label_rect =
                Rect::from_origin_and_size(Point::new(padding_h, padding_v), max_size.cast());
            let button_rect = label_rect.inflate(padding_h, padding_v);
            let button_surf = self.theme.button(button_rect.size().cast())?;
            let background =
                FramedTexture::new(button_rect, button_surf.as_texture(self.texture_creator)?);
            let label = FramedTexture::new(label_rect, label.as_texture(self.texture_creator)?);

            out.push(Button::new(background, label));
        }

        Ok(out)
    }
}
