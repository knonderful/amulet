use amulet_core::component::{
    ComponentEvent, HandleEvent, MouseSensor, MouseSensorState, Position, Render, Size, Text,
};
use amulet_core::geom::{ComponentSize, Rect};
use amulet_core::mouse::Button;
use amulet_core::render::{RenderConstraints, RenderDestination};
use amulet_core::VuiResult;
use amulet_sdl2::{event_iterator, Event};
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::path::Path;
use std::rc::Rc;

struct Label<'ttf> {
    component: Text<(Rc<Font<'ttf, 'static>>, Color)>,
}

impl<'ttf> Label<'ttf> {
    fn new(font: Rc<Font<'ttf, 'static>>, text: Cow<'static, str>) -> Self {
        let component = Text::new(text, (font, Color::RGB(0, 200, 0)));
        Self { component }
    }
}

impl<'ttf> HandleEvent for Label<'ttf> {
    type State<'a> = ();

    fn handle_event(&self, _state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.component.handle_event((), event)
    }
}

impl Size for Label<'_> {
    fn size(&self) -> ComponentSize {
        self.component.size()
    }
}

impl Render for Label<'_> {
    type State<'a> = ();

    fn render(
        &self,
        state: Self::State<'_>,
        target: (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        self.component.render(state, target)
    }
}

#[derive(Debug, Default)]
struct AppState {
    click_count: u64,
}

#[derive(Debug, Default)]
struct GuiState {
    button_state: MouseSensorState,
}

struct Gui<'a> {
    button: Position<MouseSensor<Label<'a>>>,
    clicked_label: Position<Label<'a>>,
}

impl<'a> Gui<'a> {
    fn new(app_state: &AppState, font: Rc<Font<'a, 'static>>) -> Self {
        Self {
            button: Position::new(
                (10, 10).into(),
                MouseSensor::new(Label::new(font.clone(), "Button".into())),
            ),
            clicked_label: Position::new(
                (10, 50).into(),
                Label::new(
                    font.clone(),
                    format!("Count: {}", app_state.click_count).into(),
                ),
            ),
        }
    }
}

impl HandleEvent for Gui<'_> {
    type State<'a> = &'a mut GuiState;

    fn handle_event(&self, gui_state: Self::State<'_>, event: ComponentEvent) -> VuiResult<()> {
        self.button
            .handle_event((&mut gui_state.button_state, ()), event.clone())?;
        self.clicked_label.handle_event((), event.clone())?;
        Ok(())
    }
}

impl Render for Gui<'_> {
    type State<'a> = &'a GuiState;

    fn render(
        &self,
        gui_state: Self::State<'_>,
        (dest, constriants): (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        self.button
            .render((&gui_state.button_state, ()), (dest, constriants.clone()))?;
        self.clicked_label.render((), (dest, constriants.clone()))?;
        Ok(())
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
    let font = Rc::new(ttf_context.load_font(
        Path::new("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf"),
        14,
    )?);
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = AppState::default();
    let mut gui_state = GuiState::default();

    'running: loop {
        let gui = Gui::new(&app_state, font.clone());

        for event in event_iterator(&mut event_pump) {
            match event {
                Event::Amulet(evt) => {
                    gui.handle_event(&mut gui_state, evt)?;
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

        if gui_state
            .button_state
            .click_states()
            .has_click_completed(Button::Left)
        {
            app_state.click_count += 1;
        }

        canvas.set_draw_color(Color::RGB(24, 24, 24));
        canvas.clear();

        let mut render_dest = RenderDestination::new(&texture_creator, &mut canvas);
        let constraints = RenderConstraints::new(Rect::new((0, 0).into(), (800, 600).into()));
        gui.render(&gui_state, (&mut render_dest, constraints.clone()))?;

        canvas.present();
    }

    Ok(())
}
