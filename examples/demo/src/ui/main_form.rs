use amulet_core::component::{
    AsChain, ComponentEvent, Frame, HandleEvent, Layout, Position, PositionAttr, SizeAttr,
    UpdateLayout,
};
use amulet_core::geom::{Point, Rect, Size};
use amulet_core::VuiResult;
use amulet_ez::theme::Theme;
use amulet_ez::widget::{Button, ButtonState};
use amulet_sdl2::render::{Render, RenderContext};

#[derive(Debug, Default)]
pub struct MainFormState {
    pub button: ButtonState,
    pub btn_ok: ButtonState,
    pub btn_defaults: ButtonState,
    pub btn_cancel: ButtonState,
}

pub struct MainForm<'a> {
    widget_factory: &'a Theme<'a>,
    button: (Position, Button<'a>),
    /// Anchor position for `btn_*` components.
    anchor: Position,
    btn_ok: (Position, Button<'a>),
    btn_defaults: (Position, Button<'a>),
    btn_cancel: (Position, Button<'a>),
}

trait AlignCenter {
    fn align_center(self, max_size: Size) -> (Frame, Position, Self);
}

impl<T> AlignCenter for T
where
    T: SizeAttr,
{
    fn align_center(self, max_size: Size) -> (Frame, Position, Self) {
        let position = Position::new(((max_size - self.size()) / 2).as_vector().as_point());
        (Frame::new(max_size), position, self)
    }
}

impl<'a> MainForm<'a> {
    pub fn new(theme: &'a Theme<'a>, rect: Rect, click_count: u64) -> VuiResult<Self> {
        let button = Self::create_button(theme, click_count)?;

        let lbl_ok = theme.label("OK")?;
        let lbl_defaults = theme.label("Defaults")?;
        let lbl_cancel = theme.label("Cancel")?;
        let max = lbl_ok
            .size()
            .max(lbl_defaults.size())
            .max(lbl_cancel.size());

        let spacing = 8;
        let pos = Point::zero();
        let btn_ok = (Position::new(pos), theme.button(lbl_ok.align_center(max))?);
        let pos = pos + Point::new(btn_ok.1.size().width + spacing, 0);
        let btn_defaults = (
            Position::new(pos),
            theme.button(lbl_defaults.align_center(max))?,
        );
        let pos = pos + Point::new(btn_defaults.1.size().width + spacing, 0);
        let btn_cancel = (
            Position::new(pos),
            theme.button(lbl_cancel.align_center(max))?,
        );
        let anchor = Self::calc_anchor(rect, &btn_cancel);

        Ok(Self {
            widget_factory: theme,
            button,
            anchor,
            btn_ok,
            btn_defaults,
            btn_cancel,
        })
    }

    fn create_button(theme: &'a Theme<'a>, click_count: u64) -> VuiResult<(Position, Button<'a>)> {
        let text = theme.label(&format!("EZ Button ({} clicks)", click_count))?;
        let content = (Frame::new(text.size()), Position::new(Point::zero()), text);
        Ok((Position::new((80, 100).into()), theme.button(content)?))
    }

    fn calc_anchor(rect: Rect, btn: &(Position, Button)) -> Position {
        Position::new(rect.limit() - Rect::new(btn.0.position(), btn.1.size()).limit())
    }

    pub fn resize(&mut self, rect: Rect) -> VuiResult<()> {
        self.anchor = Self::calc_anchor(rect, &self.btn_cancel);
        Ok(())
    }

    pub fn update_click_count(&mut self, click_count: u64) -> VuiResult<()> {
        self.button = Self::create_button(self.widget_factory, click_count)?;
        Ok(())
    }
}

impl HandleEvent for MainForm<'_> {
    type State<'a> = &'a mut MainFormState;

    fn handle_event(
        &self,
        gui_state: &mut MainFormState,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        self.button
            .as_chain()
            .handle_event(((), &mut gui_state.button), event.clone())?;
        let event = self.anchor.handle_event((), event)?;
        self.btn_ok
            .as_chain()
            .handle_event(((), &mut gui_state.btn_ok), event.clone())?;
        self.btn_defaults
            .as_chain()
            .handle_event(((), &mut gui_state.btn_defaults), event.clone())?;
        self.btn_cancel
            .as_chain()
            .handle_event(((), &mut gui_state.btn_cancel), event.clone())?;

        // Kind of nonsensical =)
        Ok(event)
    }
}

impl Render for MainForm<'_> {
    type State<'a> = &'a MainFormState;

    fn render(
        &self,
        gui_state: Self::State<'_>,
        layout: Layout,
        render_ctx: &mut RenderContext,
    ) -> VuiResult<()> {
        self.button
            .render(((), &gui_state.button), layout.clone(), render_ctx)?;
        let layout = self.anchor.update_layout((), layout)?;
        self.btn_ok
            .render(((), &gui_state.btn_ok), layout.clone(), render_ctx)?;
        self.btn_defaults
            .render(((), &gui_state.btn_defaults), layout.clone(), render_ctx)?;
        self.btn_cancel
            .render(((), &gui_state.btn_cancel), layout.clone(), render_ctx)?;

        Ok(())
    }
}
