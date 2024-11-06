use amulet_core::component::{
    ComponentEvent, HandleEvent, Position, Render, RenderConstraints, SizeAttr,
};
use amulet_core::geom::{Point, Rect, Size};
use amulet_core::VuiResult;
use amulet_ez::widget::{Button, ButtonState, WidgetFactory};
use amulet_sdl2::render::SdlRender;

/*
pane!{
    ButtonGroup<'a> => ButtonState {
        ok: (Position, Button<'a>) => state.ok,
        cancel: (Position, Button<'a>), // implies state.cancel
        defaults: (Position, Button<'a>), // implies state.defaults
    }
}
 */

struct ButtonGroup<'a> {
    btn_ok: (Position, Button<'a>),
    btn_cancel: (Position, Button<'a>),
    btn_defaults: (Position, Button<'a>),
}

impl<'a> ButtonGroup<'a> {
    pub fn new(widget_factory: &mut WidgetFactory<'a>) -> VuiResult<Self> {
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

        Ok(Self {
            btn_ok,
            btn_cancel,
            btn_defaults,
        })
    }
}

impl SizeAttr for ButtonGroup<'_> {
    fn size(&self) -> Size {
        Rect::new(self.btn_cancel.0.position(), self.btn_cancel.1.size())
            .limit()
            .as_size()
    }
}

impl HandleEvent for ButtonGroup<'_> {
    type State<'a> = (
        &'a mut ButtonState,
        &'a mut ButtonState,
        &'a mut ButtonState,
    );

    fn handle_event(
        &self,
        state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        let (state_ok, state_cancel, state_defaults) = state;
        self.btn_ok.handle_event(((), state_ok), event.clone())?;
        self.btn_cancel
            .handle_event(((), state_cancel), event.clone())?;
        self.btn_defaults
            .handle_event(((), state_defaults), event.clone())
    }
}

impl<R> Render<R> for ButtonGroup<'_>
where
    R: SdlRender,
{
    type State<'a> = (&'a ButtonState, &'a ButtonState, &'a ButtonState);

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        let (state_ok, state_cancel, state_defaults) = state;
        self.btn_ok
            .render(((), state_ok), constraints.clone(), render_ctx)?;
        self.btn_cancel
            .render(((), state_cancel), constraints.clone(), render_ctx)?;
        self.btn_defaults
            .render(((), state_defaults), constraints.clone(), render_ctx)
    }
}

#[derive(Debug, Default)]
pub struct MainFormState {
    pub button: ButtonState,
    pub btn_ok: ButtonState,
    pub btn_cancel: ButtonState,
    pub btn_defaults: ButtonState,
}

pub struct MainForm<'a> {
    widget_factory: &'a mut WidgetFactory<'a>,
    button: (Position, Button<'a>),
    button_group: (Position, ButtonGroup<'a>),
}

impl<'a> MainForm<'a> {
    pub fn new(
        widget_factory: &'a mut WidgetFactory<'a>,
        rect: Rect,
        click_count: u64,
    ) -> VuiResult<Self> {
        let button = Self::create_button(widget_factory, click_count)?;
        let button_group = ButtonGroup::new(widget_factory)?;
        let pos = rect.limit() - button_group.size().as_vector();
        let button_group = (Position::new(pos), button_group);

        Ok(Self {
            widget_factory,
            button,
            button_group,
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

    pub fn resize(&mut self, rect: Rect) -> VuiResult<()> {
        let pos = rect.limit() - self.button_group.1.size().as_vector();
        self.button_group.0 = Position::new(pos);
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
        self.button_group.handle_event(
            (
                (),
                (
                    &mut gui_state.btn_ok,
                    &mut gui_state.btn_cancel,
                    &mut gui_state.btn_defaults,
                ),
            ),
            event.clone(),
        )?;

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
        self.button_group.render(
            (
                (),
                (
                    &gui_state.btn_ok,
                    &gui_state.btn_cancel,
                    &gui_state.btn_defaults,
                ),
            ),
            constraints.clone(),
            render_ctx,
        )?;

        // Kind of nonsensical =)
        Ok(constraints)
    }
}
