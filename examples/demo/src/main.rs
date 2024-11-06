use amulet_core::component::{ComponentEvent, HandleEvent, Position, Render, RenderConstraints};
use amulet_core::geom::{Rect, Size};
use amulet_core::VuiResult;
use amulet_ez::theme::Theme;
use amulet_ez::widget::{Button, ButtonState, WidgetFactory};
use amulet_sdl2::render::{RenderContext, SdlRender};
use amulet_sdl2::{event_iterator, Event};
use sdl2::event::{Event as SdlEvent, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

#[derive(Debug, Default)]
struct AppState {
    click_count: u64,
}

#[derive(Debug, Default)]
struct MainFormState {
    button: ButtonState,
    btn_ok: ButtonState,
    btn_cancel: ButtonState,
    btn_defaults: ButtonState,
}

struct MainForm<'a> {
    widget_factory: &'a mut WidgetFactory<'a>,
    button: (Position, Button<'a>),
    btn_ok: (Position, Button<'a>),
    btn_cancel: (Position, Button<'a>),
    btn_defaults: (Position, Button<'a>),
}

impl<'a> MainForm<'a> {
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
        let mut buttons = widget_factory
            .button_set(&["OK", "Cancel", "Defaults"])?
            .into_iter();
        let btn_ok = (Position::new((10, 100).into()), buttons.next().unwrap());
        let btn_cancel = (Position::new((10, 130).into()), buttons.next().unwrap());
        let btn_defaults = (Position::new((10, 160).into()), buttons.next().unwrap());
        Ok(Self {
            widget_factory,
            button,
            btn_ok,
            btn_cancel,
            btn_defaults,
        })
    }

    fn update(&mut self, click_count: u64) -> VuiResult<()> {
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
        for (c, mut s) in [
            (&self.button, &mut gui_state.button),
            (&self.btn_ok, &mut gui_state.btn_ok),
            (&self.btn_cancel, &mut gui_state.btn_cancel),
            (&self.btn_defaults, &mut gui_state.btn_defaults),
        ] {
            c.handle_event(((), &mut s), event.clone())?;
        }
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
        for (c, s) in [
            (&self.button, &gui_state.button),
            (&self.btn_ok, &gui_state.btn_ok),
            (&self.btn_cancel, &gui_state.btn_cancel),
            (&self.btn_defaults, &gui_state.btn_defaults),
        ] {
            c.render(((), &s), constraints.clone(), render_ctx)?;
        }
        // Kind of nonsensical =)
        Ok(constraints)
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

    let mut window_rect = {
        let (w, h) = window.size();
        Rect::from_size(Size::new(w, h).cast())
    };

    let mut canvas = window.into_canvas().present_vsync().build()?;

    let ttf_context = sdl2::ttf::init()?;
    let texture_creator = canvas.texture_creator();
    let theme = Theme::create(&ttf_context)?;
    let mut widget_factory = WidgetFactory::new(&theme, &texture_creator);

    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = AppState::default();
    let mut main_form_state = MainFormState::default();

    let mut main_form = MainForm::new(&mut widget_factory, app_state.click_count)?;

    'running: loop {
        for event in event_iterator(&mut event_pump) {
            match event {
                Event::Amulet(evt) => {
                    main_form.handle_event(
                        &mut main_form_state,
                        evt.into_component_event(window_rect),
                    )?;
                }
                Event::Sdl(evt) => match evt {
                    SdlEvent::Quit { .. }
                    | SdlEvent::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    SdlEvent::Window { win_event, .. } => match win_event {
                        WindowEvent::SizeChanged(x, y) => window_rect.set_size((x, y).into()),
                        WindowEvent::Resized(x, y) => window_rect.set_size((x, y).into()),
                        _ => {}
                    },
                    _ => {}
                },
            }
        }

        if main_form_state.button.was_clicked() {
            app_state.click_count += 1;
        }

        canvas.set_draw_color(Color::RGB(0x3c, 0x3f, 0x41));
        canvas.clear();

        let mut render_ctx = RenderContext::new(&mut canvas);
        let constraints = RenderConstraints::new(window_rect);
        main_form.render(&main_form_state, constraints, &mut render_ctx)?;

        canvas.present();

        main_form.update(app_state.click_count)?;
    }

    Ok(())
}
