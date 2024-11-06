use amulet_core::geom::{Rect, Size};
use amulet_core::{VuiError, VuiResult};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct Theme<'a> {
    #[allow(unused)]
    font: Font<'a, 'static>,
}

impl<'a> Theme<'a> {
    pub fn create(ttf: &'a Sdl2TtfContext) -> VuiResult<Self> {
        let font = ttf
            .load_font("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf", 14)
            .map_err(|e| VuiError::new(format!("load_font() error: {e}")))?;

        Ok(Self { font })
    }
}

const PRIMARY_FG: Color = Color::RGB(0xbb, 0xbb, 0xbb);
const PRIMARY_BG: Color = Color::RGB(0x4d, 0x51, 0x53);
const PRIMARY_EDGE: Color = Color::RGB(0x5f, 0x61, 0x61);

trait DrawRect {
    fn draw_amu_rect(&mut self, rect: Rect) -> VuiResult<()>;
}

impl<T> DrawRect for Canvas<T>
where
    T: RenderTarget,
{
    fn draw_amu_rect(&mut self, rect: Rect) -> VuiResult<()> {
        let points = {
            let (x, y) = rect.max.into();
            [
                sdl2::rect::Point::new(0, 0),
                sdl2::rect::Point::new(x, 0),
                sdl2::rect::Point::new(x, y),
                sdl2::rect::Point::new(0, y),
                sdl2::rect::Point::new(0, 0),
            ]
        };
        self.draw_lines(points.as_slice())?;
        Ok(())
    }
}

trait CanvasExt {
    fn draw_border(&mut self, rect: Rect) -> VuiResult<()>;
}

impl<T> CanvasExt for Canvas<T>
where
    T: RenderTarget,
{
    fn draw_border(&mut self, rect: Rect) -> VuiResult<()> {
        let (x, y) = rect.max.into();
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
    pub fn label(&self, text: &str) -> VuiResult<Surface<'static>> {
        Ok(self.font.render(text).blended(PRIMARY_FG)?)
    }

    pub fn button(&self, size: Size) -> VuiResult<Surface<'static>> {
        let surface = Surface::new(size.width, size.height, PixelFormatEnum::RGB888)?;
        let mut canvas = surface.into_canvas()?;
        canvas.set_draw_color(PRIMARY_BG);
        canvas.clear();

        canvas.set_draw_color(PRIMARY_EDGE);
        canvas.draw_border(Rect::from_size(size.cast()).inflate(-1, -1))?;
        Ok(canvas.into_surface())
    }
}
