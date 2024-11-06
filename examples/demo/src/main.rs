use amulet_core::component::{ComponentEvent, HandleEvent, Position, Render, RenderConstraints};
use amulet_core::geom::Rect;
use amulet_core::VuiResult;
use amulet_ez::theme::Theme;
use amulet_ez::widget::{Button, ButtonState, WidgetFactory};
use amulet_sdl2::render::{RenderContext, SdlRender};
use amulet_sdl2::{event_iterator, Event};
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

#[derive(Debug, Default)]
struct AppState {
    click_count: u64,
}

#[derive(Debug, Default)]
struct GuiState {
    button_state: ButtonState,
}

struct Gui<'a> {
    widget_factory: &'a mut WidgetFactory<'a>,
    button: (Position, Button<'a>),
}

impl<'a> Gui<'a> {
    fn create_button(
        widget_factory: &mut WidgetFactory<'a>,
        click_count: u64,
    ) -> VuiResult<(Position, Button<'a>)> {
        Ok((
            Position::new((30, 20).into()),
            widget_factory.button(&format!("EZ Button ({} clicks)", click_count))?,
        ))
    }

    fn new(widget_factory: &'a mut WidgetFactory<'a>, _click_count: u64) -> VuiResult<Self> {
        let button = Self::create_button(widget_factory, _click_count)?;
        Ok(Self {
            widget_factory,
            button,
        })
    }

    fn update(&mut self, click_count: u64) -> VuiResult<()> {
        self.button = Self::create_button(self.widget_factory, click_count)?;
        Ok(())
    }
}

impl HandleEvent for Gui<'_> {
    type State<'a> = &'a mut GuiState;

    fn handle_event(
        &self,
        gui_state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        self.button
            .handle_event(((), &mut gui_state.button_state), event)
    }
}

impl<R> Render<R> for Gui<'_>
where
    R: SdlRender,
{
    type State<'a> = &'a GuiState;

    fn render(
        &self,
        gui_state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<RenderConstraints> {
        self.button.render(
            ((), &gui_state.button_state),
            constraints.clone(),
            render_ctx,
        )
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("VUI demo", 800, 600)
        .position_centered()
        .resizable()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;

    let ttf_context = sdl2::ttf::init()?;
    let texture_creator = canvas.texture_creator();
    let theme = Theme::create(&ttf_context)?;
    let mut widget_factory = WidgetFactory::new(&theme, &texture_creator);

    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = AppState::default();
    let mut gui_state = GuiState::default();
    let comp_rect = Rect::from_size((800, 600).into());

    let mut gui = Gui::new(&mut widget_factory, app_state.click_count)?;

    'running: loop {
        for event in event_iterator(&mut event_pump) {
            match event {
                Event::Amulet(evt) => {
                    gui.handle_event(&mut gui_state, evt.into_component_event(comp_rect))?;
                }
                Event::Sdl(evt) => match evt {
                    SdlEvent::Quit { .. }
                    | SdlEvent::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                },
            }
        }

        if gui_state.button_state.was_clicked() {
            app_state.click_count += 1;
        }

        canvas.set_draw_color(Color::RGB(0x3c, 0x3f, 0x41));
        canvas.clear();

        let mut render_ctx = RenderContext::new(&mut canvas);
        let constraints = RenderConstraints::new(Rect::new((0, 0).into(), (800, 600).into()));
        gui.render(&gui_state, constraints, &mut render_ctx)?;

        canvas.present();

        gui.update(app_state.click_count)?;
    }

    Ok(())
}
