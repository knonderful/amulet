use crate::ui::main_form::{MainForm, MainFormState};
use amulet_core::component::{HandleEvent, Layout};
use amulet_core::geom::Rect;
use amulet_ez::theme::Theme;
use amulet_sdl2::lossy::LossyInto;
use amulet_sdl2::render::{Render, RenderContext};
use amulet_sdl2::{event_iterator, Event};
use sdl2::event::{Event as SdlEvent, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::ops::{Deref, DerefMut};

mod ui;

#[derive(Debug, Default)]
struct AppState {
    click_count: u64,
}

#[derive(Debug, Default)]
struct ChangeDetector<T>(T, bool);

impl<T> ChangeDetector<T> {
    pub fn new(inner: T) -> Self {
        Self(inner, false)
    }

    pub fn changed(&mut self) -> bool {
        let out = self.1;
        self.1 = false;
        out
    }
}

impl<T> Deref for ChangeDetector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ChangeDetector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1 = true;
        &mut self.0
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Amulet Demo", 800, 600)
        .position_centered()
        .resizable()
        .build()?;

    let mut window_rect = ChangeDetector::new({
        let size: (i32, i32) = window.size().lossy_into();
        Rect::from_size(size.into())
    });

    let mut canvas = window.into_canvas().present_vsync().build()?;

    let ttf_context = sdl2::ttf::init()?;
    let texture_creator = canvas.texture_creator();
    let theme = Theme::create(&ttf_context, texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = ChangeDetector::new(AppState::default());
    let mut main_form_state = MainFormState::default();

    let mut main_form = MainForm::new(&theme, *window_rect, app_state.click_count)?;

    'running: loop {
        for event in event_iterator(&mut event_pump) {
            match event {
                Event::Amulet(evt) => {
                    main_form.handle_event(
                        &mut main_form_state,
                        evt.into_component_event(*window_rect),
                    )?;
                }
                Event::Sdl(evt) => match evt {
                    SdlEvent::Quit { .. }
                    | SdlEvent::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    SdlEvent::Window { win_event, .. } => match win_event {
                        WindowEvent::SizeChanged(x, y) => {
                            *window_rect = window_rect.resize((x, y).into());
                        }
                        WindowEvent::Resized(x, y) => {
                            *window_rect = window_rect.resize((x, y).into());
                        }
                        _ => {}
                    },
                    _ => {}
                },
            }
        }

        if main_form_state.button.was_clicked() {
            app_state.click_count += 1;
        }
        if main_form_state.btn_cancel.was_clicked() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGB(0x3c, 0x3f, 0x41));
        canvas.clear();

        let mut render_ctx = RenderContext::new(&mut canvas);
        let layout = Layout::new(*window_rect);
        main_form.render(&main_form_state, layout, &mut render_ctx)?;

        canvas.present();

        if app_state.changed() {
            main_form.update_click_count(app_state.click_count)?;
        }
        if window_rect.changed() {
            main_form.resize(*window_rect)?;
        }
    }

    Ok(())
}
