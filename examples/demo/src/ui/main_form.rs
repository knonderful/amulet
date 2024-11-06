use amulet_core::component::{
    ComponentEvent, HandleEvent, Position, PositionAttr, Render, RenderConstraints, SizeAttr,
};
use amulet_core::geom::{Point, Rect};
use amulet_core::VuiResult;
use amulet_ez::widget::{Button, ButtonState, WidgetFactory};
use amulet_sdl2::render::SdlRender;

#[derive(Debug, Default)]
pub struct MainFormState {
    pub button: ButtonState,
    pub btn_ok: ButtonState,
    pub btn_defaults: ButtonState,
    pub btn_cancel: ButtonState,
}

pub struct MainForm<'a> {
    widget_factory: &'a mut WidgetFactory<'a>,
    button: (Position, Button<'a>),
    /// Anchor position for `btn_*` components.
    anchor: Position,
    btn_ok: (Position, Button<'a>),
    btn_defaults: (Position, Button<'a>),
    btn_cancel: (Position, Button<'a>),
}

impl<'a> MainForm<'a> {
    pub fn new(
        widget_factory: &'a mut WidgetFactory<'a>,
        rect: Rect,
        click_count: u64,
    ) -> VuiResult<Self> {
        let button = Self::create_button(widget_factory, click_count)?;
        let mut buttons = widget_factory
            .button_set(&["OK", "Defaults", "Cancel"])?
            .into_iter();

        let spacing = 8;
        let pos = Point::zero();
        let btn_ok = (Position::new(pos), buttons.next().unwrap());
        let pos = pos + Point::new(btn_ok.1.size().width + spacing, 0);
        let btn_defaults = (Position::new(pos), buttons.next().unwrap());
        let pos = pos + Point::new(btn_defaults.1.size().width + spacing, 0);
        let btn_cancel = (Position::new(pos), buttons.next().unwrap());
        let anchor = Self::calc_anchor(rect, &btn_cancel);

        Ok(Self {
            widget_factory,
            button,
            anchor,
            btn_ok,
            btn_defaults,
            btn_cancel,
        })
    }

    fn create_button(
        widget_factory: &mut WidgetFactory<'a>,
        click_count: u64,
    ) -> VuiResult<(Position, Button<'a>)> {
        Ok((
            Position::new((80, 100).into()),
            widget_factory.button(&format!("EZ Button ({} clicks)", click_count))?,
        ))
    }

    fn calc_anchor(rect: Rect, btn: &(Position, Button<'a>)) -> Position {
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
            .handle_event(((), &mut gui_state.button), event.clone())?;
        let event = self.anchor.handle_event((), event)?;
        self.btn_ok
            .handle_event(((), &mut gui_state.btn_ok), event.clone())?;
        self.btn_defaults
            .handle_event(((), &mut gui_state.btn_defaults), event.clone())?;
        self.btn_cancel
            .handle_event(((), &mut gui_state.btn_cancel), event.clone())?;

        // Kind of nonsensical =)
        Ok(event)
    }
}

impl<R> Render<R> for MainForm<'_>
where
    R: SdlRender,
{
    type State<'a> = &'a MainFormState;

    fn render(
        &self,
        gui_state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        self.button
            .render(((), &gui_state.button), constraints.clone(), render_ctx)?;
        let constraints = self.anchor.render((), constraints, render_ctx)?;
        self.btn_ok
            .render(((), &gui_state.btn_ok), constraints.clone(), render_ctx)?;
        self.btn_defaults.render(
            ((), &gui_state.btn_defaults),
            constraints.clone(),
            render_ctx,
        )?;
        self.btn_cancel
            .render(((), &gui_state.btn_cancel), constraints.clone(), render_ctx)?;

        // Kind of nonsensical =)
        Ok(constraints)
    }
}
