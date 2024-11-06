use crate::widget::{Button, Image};
use amulet_core::component::{Frame, MouseSensor, Position};
use amulet_core::geom::{Rect, Size};
use amulet_core::{VuiError, VuiResult};
use amulet_sdl2::lossy::LossyInto;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, RenderTarget, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::WindowContext;
use std::rc::Rc;

pub struct Theme<'a> {
    font: Font<'a, 'static>,
    texture_creator: TextureCreator<WindowContext>,
}

impl<'a> Theme<'a> {
    pub fn create(
        ttf: &'a Sdl2TtfContext,
        texture_creator: TextureCreator<WindowContext>,
    ) -> VuiResult<Self> {
        let font = ttf
            .load_font("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf", 14)
            .map_err(|e| VuiError::new(format!("load_font() error: {e}")))?;

        Ok(Self {
            font,
            texture_creator,
        })
    }
}

const PRIMARY_FG: Color = Color::RGB(0xbb, 0xbb, 0xbb);
const PRIMARY_BG: Color = Color::RGB(0x4d, 0x51, 0x53);
const PRIMARY_EDGE: Color = Color::RGB(0x5f, 0x61, 0x61);
const PADDING_H: i32 = 5;
const PADDING_V: i32 = 3;

trait CanvasExt {
    fn draw_border(&mut self, rect: Rect) -> VuiResult<()>;
}

impl<T> CanvasExt for Canvas<T>
where
    T: RenderTarget,
{
    fn draw_border(&mut self, rect: Rect) -> VuiResult<()> {
        let (x, y) = rect.limit().into();
        self.draw_line(
            sdl2::rect::Point::new(1, 0),
            sdl2::rect::Point::new(x - 1, 0),
        )?;
        self.draw_line(
            sdl2::rect::Point::new(x, 1),
            sdl2::rect::Point::new(x, y - 1),
        )?;
        self.draw_line(
            sdl2::rect::Point::new(1, y),
            sdl2::rect::Point::new(x - 1, y),
        )?;
        self.draw_line(
            sdl2::rect::Point::new(0, 1),
            sdl2::rect::Point::new(0, y - 1),
        )?;
        Ok(())
    }
}

impl Theme<'_> {
    pub fn label(&self, text: &str) -> VuiResult<Image> {
        let surf = self.font.render(text).blended(PRIMARY_FG)?;
        let size: (i32, i32) = surf.size().lossy_into();
        let texture = Rc::new(surf.as_texture(&self.texture_creator)?);
        Ok(Image::new(texture, size.into()))
    }

    pub fn button(&self, content_size: Size) -> VuiResult<Button> {
        let button_size = content_size + Size::new(PADDING_H * 2, PADDING_V * 2);
        let surface = Surface::new(
            button_size.width.lossy_into(),
            button_size.height.lossy_into(),
            PixelFormatEnum::RGB888,
        )?;

        let mut bg_canvas = surface.into_canvas()?;
        bg_canvas.set_draw_color(PRIMARY_BG);
        bg_canvas.clear();

        bg_canvas.set_draw_color(PRIMARY_EDGE);
        bg_canvas.draw_border(Rect::from_size(button_size).inflate(-1, -1))?;

        let bg_image = Image::new(
            Rc::new(bg_canvas.into_surface().as_texture(&self.texture_creator)?),
            button_size,
        );

        let component = (
            Frame::new(button_size),
            MouseSensor::new(),
            bg_image,
            Position::new((PADDING_H, PADDING_V).into()),
            Frame::new(content_size),
        );

        Ok(Button { component })
    }
}
