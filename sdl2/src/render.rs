use crate::lossy::LossyInto;
use amulet_core::component::{AdjustLayout, Layout};
use amulet_core::VuiResult;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

pub trait Render {
    type State<'a>;

    fn render(
        &self,
        state: Self::State<'_>,
        layout: Layout,
        render_context: &mut RenderContext,
    ) -> VuiResult<()>;
}

impl<A, Z> Render for (A, Z)
where
    A: AdjustLayout,
    Z: Render,
{
    type State<'a> = (A::State<'a>, Z::State<'a>);

    fn render(
        &self,
        state: Self::State<'_>,
        layout: Layout,
        render_context: &mut RenderContext,
    ) -> VuiResult<()> {
        let (a, z) = self;
        let (a_state, z_state) = state;
        let layout = a.adjust_layout(a_state, layout)?;
        z.render(z_state, layout, render_context)
    }
}

pub struct RenderContext<'a> {
    canvas: &'a mut WindowCanvas,
}

impl<'a> RenderContext<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Self {
        Self { canvas }
    }

    pub fn get_canvas(&mut self, layout: Layout) -> &mut Canvas<Window> {
        let rect = layout.clip_rect();
        let (x, y, w, h) = rect.into();
        self.canvas
            .set_viewport(sdl2::rect::Rect::new(x, y, w.lossy_into(), h.lossy_into()));
        self.canvas
    }
}
