use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use std::borrow::Cow;
use std::path::PathBuf;
use std::rc::Rc;
use vui_core::component::mouse_aware::{MouseAware, MouseAwareState};
use vui_core::component::{ComponentEvent, HandleEvent, MouseButton, Pos, Render, Size, Text};
use vui_core::font_manager::{FontDetails, FontManager};
use vui_core::render::{RenderConstraints, RenderDestination};
use vui_core::VuiResult;

struct Label<'ttf> {
    component: Text<(Rc<Font<'ttf, 'static>>, Color)>,
}

impl<'ttf> Label<'ttf> {
    fn new(font: Rc<Font<'ttf, 'static>>, text: Cow<'static, str>) -> Self {
        let component = Text::new(text, (font, Color::RGB(0, 200, 0)));
        Self { component }
    }
}

impl HandleEvent for Label<'_> {
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        self.component.handle_event(event)
    }
}

impl Size for Label<'_> {
    fn size(&self) -> vui_core::math::Size {
        self.component.size()
    }
}

impl Render for Label<'_> {
    fn render(&self, target: (&mut RenderDestination, RenderConstraints)) -> VuiResult<()> {
        self.component.render(target)
    }
}

#[derive(Debug, Default)]
struct AppState {
    click_count: u64,
}

#[derive(Debug, Default)]
struct GuiState {
    button_state: MouseAwareState,
}

struct Gui<'a> {
    button: Pos<MouseAware<&'a mut MouseAwareState, Label<'a>>>,
    clicked_label: Pos<Label<'a>>,
}

impl<'a> Gui<'a> {
    fn new<'b>(
        app_state: &AppState,
        gui_state: &'a mut GuiState,
        font: Rc<Font<'a, 'static>>,
    ) -> Self {
        Self {
            button: Pos::new(
                (10, 10).into(),
                MouseAware::new(
                    &mut gui_state.button_state,
                    Label::new(font.clone(), "Button".into()),
                ),
            ),
            clicked_label: Pos::new(
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
    fn handle_event(&mut self, event: ComponentEvent) -> VuiResult<()> {
        for comp in [
            &mut self.button,
            &mut self.clicked_label as &mut dyn HandleEvent,
        ] {
            comp.handle_event(event.clone())?
        }
        Ok(())
    }
}

impl Render for Gui<'_> {
    fn render(
        &self,
        (dest, constriants): (&mut RenderDestination, RenderConstraints),
    ) -> VuiResult<()> {
        for comp in [&self.button, &self.clicked_label as &dyn Render] {
            comp.render((dest, constriants.clone()))?
        }
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
    let mut font_manager = FontManager::new(&ttf_context);
    let font = font_manager.load(&FontDetails {
        path: PathBuf::from("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf"),
        size: 14,
    })?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = AppState::default();
    let mut gui_state = GuiState::default();

    'running: loop {
        let mut gui = Gui::new(&app_state, &mut gui_state, font.clone());
        // let mut view = View::new(&mut gui);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    gui.handle_event(ComponentEvent::MouseMotion((x, y).into()))?;
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let Ok(btn) = mouse_btn.try_into() else {
                        continue;
                    };
                    gui.handle_event(ComponentEvent::MouseButtonUp(btn, (x, y).into()))?;
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let Ok(btn) = mouse_btn.try_into() else {
                        continue;
                    };
                    gui.handle_event(ComponentEvent::MouseButtonDown(btn, (x, y).into()))?;
                }
                _ => {}
            }
        }
        // NB: Currently we have the gui_state and the app_state tied to the gui, which means we can't use them unless
        //     we drop the gui first. This means we have to rebuild the gui for the render (or pay the price of always
        //     having a state-lag of 1 frame. =(
        //     The solution would be to NOT tie the gui state to the gui, which means somehow passing the state through
        //     handle_event(), which means each type needs to specify its state (cascadingly).
        drop(gui);

        if gui_state.button_state.click_completed(MouseButton::Left) {
            app_state.click_count += 1;
        }

        canvas.set_draw_color(Color::RGB(24, 24, 24));
        canvas.clear();

        let mut render_dest = RenderDestination::new(&texture_creator, &mut canvas);
        let constraints = RenderConstraints::new(Rect::new(0, 0, 800, 600));
        let gui = Gui::new(&app_state, &mut gui_state, font.clone());
        gui.render((&mut render_dest, constraints.clone()))?;

        canvas.present();
    }

    Ok(())
}
