use amulet_core::component::{
    CalculateSize, ComponentEvent, Frame, HandleEvent, MouseSensor, MouseSensorState, Position,
    Render, RenderConstraints, Stack,
};
use amulet_core::geom::{Rect, Size};
use amulet_core::mouse::Button as MouseButton;
use amulet_core::VuiResult;
use amulet_sdl2::render::{RenderContext, SdlRender};
use amulet_sdl2::temp_components::Text;
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

impl CalculateSize for Label<'_> {
    fn calculate_size(&self) -> Size {
        self.component.calculate_size()
    }
}

impl<R> Render<R> for Label<'_>
where
    R: SdlRender,
{
    type State<'a> = ();

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        self.component.render(state, constraints, render_ctx)
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
    font: Rc<Font<'a, 'static>>,
    button: (Position, (Frame, (MouseSensor, Label<'a>))),
    clicked_label: (Position, Label<'a>),
    // ez_button: amulet_ez::widget::Button,
}

impl<'a> Gui<'a> {
    fn create_clicked_label(
        font: Rc<Font<'a, 'static>>,
        click_count: u64,
    ) -> (Position, Label<'a>) {
        Label::new(font.clone(), format!("Count: {}", click_count).into())
            .stack(Position::new((10, 50).into()))
    }

    fn new(app_state: &AppState, font: Rc<Font<'a, 'static>>) -> Self {
        let label = Label::new(font.clone(), "Button".into());
        let size = label.calculate_size();

        let button = label
            .stack(MouseSensor::new())
            .stack(Frame::new(size))
            .stack(Position::new((10, 10).into()));

        Self {
            font: font.clone(),
            button,
            clicked_label: Self::create_clicked_label(font, app_state.click_count),
        }
    }

    fn update(&mut self, click_count: u64) {
        self.clicked_label = Self::create_clicked_label(self.font.clone(), click_count)
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

impl<R> Render<R> for Gui<'_>
where
    R: SdlRender,
{
    type State<'a> = &'a GuiState;

    fn render(
        &self,
        state: Self::State<'_>,
        constraints: RenderConstraints,
        render_ctx: &mut R,
    ) -> VuiResult<()> {
        self.button
            .render((&state.button_state, ()), constraints.clone(), render_ctx)?;
        self.clicked_label.render((), constraints, render_ctx)?;
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
    let comp_rect = Rect::from_size((800, 600).into());

    let mut gui = Gui::new(&app_state, font.clone());

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

        if gui_state
            .button_state
            .click_states()
            .has_click_completed(MouseButton::Left)
        {
            app_state.click_count += 1;
        }

        canvas.set_draw_color(Color::RGB(24, 24, 24));
        canvas.clear();

        let mut render_ctx = RenderContext::new(&texture_creator, &mut canvas);
        let constraints = RenderConstraints::new(Rect::new((0, 0).into(), (800, 600).into()));
        gui.render(&gui_state, constraints, &mut render_ctx)?;

        canvas.present();

        gui.update(app_state.click_count);
    }

    Ok(())
}
