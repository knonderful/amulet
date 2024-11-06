use amulet_core::geom::Size;
use amulet_core::{VuiError, VuiResult};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct Theme<'a> {
    #[allow(unused)]
    font: Font<'a, 'static>,
}

impl<'a> Theme<'a> {
    pub fn create(ttf: &'a mut Sdl2TtfContext) -> VuiResult<Self> {
        let font = ttf
            .load_font("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf", 14)
            .map_err(|e| VuiError::new(format!("load_font() error: {e}")))?;

        Ok(Self { font })
    }
}

const PRIMARY_FG: Color = Color::RGB(0xf8, 0xf8, 0xf2);
const PRIMARY_BG: Color = Color::RGB(0x41, 0x44, 0x50);

impl Theme<'_> {
    pub fn create_button<'a>(&self, size: Size) -> VuiResult<Surface<'static>> {
        let surface = Surface::new(size.width, size.height, PixelFormatEnum::RGB888)?;
        let mut canvas = surface.into_canvas()?;
        canvas.set_draw_color(PRIMARY_BG);
        canvas.clear();

        let w = size.width.try_into()?;
        let h = size.width.try_into()?;
        canvas.box_(0, 0, w, h, PRIMARY_FG)?;
        Ok(canvas.into_surface())
    }
}
