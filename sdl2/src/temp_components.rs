use crate::render::SdlRender;
use amulet_core::component::{
    CalculateSize, ComponentEvent, HandleEvent, Render, RenderConstraints,
};
use amulet_core::geom::Size;
use amulet_core::VuiResult;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::rc::Rc;

pub trait TextRenderer {
    fn size_of(&self, text: &str) -> VuiResult<Size>;
    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>>;
}

impl TextRenderer for (&Font<'_, '_>, Color) {
    fn size_of(&self, text: &str) -> VuiResult<Size> {
        Ok(self.0.size_of(text).map(Size::from)?)
    }

    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>> {
        Ok(self.0.render(text).blended(self.1)?)
    }
}

impl TextRenderer for (Rc<Font<'_, '_>>, Color) {
    fn size_of(&self, text: &str) -> VuiResult<Size> {
        (self.0.as_ref(), self.1).size_of(text)
    }

    fn render<'a>(&self, text: &str) -> VuiResult<Surface<'a>> {
        (self.0.as_ref(), self.1).render(text)
    }
}

pub struct Text<R> {
    text: Cow<'static, str>,
    renderer: R,
}

impl<R> Text<R> {
    pub fn new(text: Cow<'static, str>, renderer: R) -> Self {
        Self { text, renderer }
    }
}

impl<R> HandleEvent for Text<R>
where
    R: TextRenderer,
{
    type State<'a> = ();

    fn handle_event(&self, _state: Self::State<'_>, _event: ComponentEvent) -> VuiResult<()> {
        Ok(())
    }
}

impl<R> CalculateSize for Text<R>
where
    R: TextRenderer,
{
    fn calculate_size(&self) -> Size {
        self.renderer.size_of(&self.text).unwrap() // TODO: components should always know their size
    }
}

impl<R, X> Render<X> for Text<R>
where
    R: TextRenderer,
    X: SdlRender,
{
    type State<'a> = ();

    fn render(
        &self,
        _: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut X,
    ) -> VuiResult<()> {
        let surface = self.renderer.render(&self.text)?;
        render_ctx.blit_surface(constraints, &surface)
    }
}
