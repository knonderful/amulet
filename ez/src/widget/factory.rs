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
        let (padding_h, padding_v) = (5, 3);
        let label_rect =
            Rect::from_origin_and_size(Point::new(padding_h, padding_v), label_size.cast());
        let button_rect = label_rect.inflate(padding_h, padding_v);
        let button = self.theme.button(button_rect.size().cast())?;

        let button_unclicked =
            FramedTexture::new(button_rect, button.as_texture(self.texture_creator)?);
        let button_clicked = button_unclicked.clone();
        let label = FramedTexture::new(label_rect, label.as_texture(self.texture_creator)?);

        Ok(Button::new(button_unclicked, button_clicked, label))
    }
}
