use amulet_core::component::{ComponentEvent, HandleEvent, Position, Render, RenderConstraints};
use amulet_core::geom::{Point, Rect, Size, Vector};
use amulet_core::VuiResult;
use amulet_ez::theme::Theme;
use amulet_ez::widget::{Button, ButtonState, WidgetFactory};
use amulet_sdl2::render::{RenderContext, SdlRender};
use amulet_sdl2::{event_iterator, Event};
use sdl2::event::{Event as SdlEvent, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::ops::{Deref, DerefMut};

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
            Position::new((80, 100).into()),
            widget_factory.button(&format!("EZ Button ({} clicks)", click_count))?,
        ))
    }

    fn calc_anchor(rect: Rect, size: Size) -> Position {
        let (dx, dy) = size.to_i32().into();
        Position::new((rect.max - Point::new(dx, dy)).to_point())
    }

    fn anchor_bottom_right(rect: Rect, component: &mut (Position, Button)) -> Point {
        let pos = Self::calc_anchor(rect, component.1.size());
        let out = pos.position();
        component.0 = pos;
        out
    }

    fn new(
        widget_factory: &'a mut WidgetFactory<'a>,
        rect: Rect,
        click_count: u64,
    ) -> VuiResult<Self> {
        let button = Self::create_button(widget_factory, click_count)?;
        let mut buttons = widget_factory
            .button_set(&["OK", "Cancel", "Defaults"])?
            .into_iter();

        // Actual positions will be calculated in `resize()`
        let btn_ok = (Position::new(Point::zero()), buttons.next().unwrap());
        let btn_cancel = (Position::new(Point::zero()), buttons.next().unwrap());
        let btn_defaults = (Position::new(Point::zero()), buttons.next().unwrap());

        let mut form = Self {
            widget_factory,
            button,
            btn_ok,
            btn_cancel,
            btn_defaults,
        };

        form.resize(rect)?;

        Ok(form)
    }

    fn resize(&mut self, rect: Rect) -> VuiResult<()> {
        let pos = Self::anchor_bottom_right(rect, &mut self.btn_ok);
        let rect = rect.translate(Vector::new(pos.x - rect.size().width - 10, 0));
        let pos = Self::anchor_bottom_right(rect, &mut self.btn_defaults);
        let rect = rect.translate(Vector::new(pos.x - rect.size().width - 10, 0));
        Self::anchor_bottom_right(rect, &mut self.btn_cancel);

        Ok(())
    }

    fn update_click_count(&mut self, click_count: u64) -> VuiResult<()> {
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

    let mut window_rect = ChangeDetector::new({
        let (w, h) = window.size();
        Rect::from_size(Size::new(w, h).cast())
    });

    let mut canvas = window.into_canvas().present_vsync().build()?;

    let ttf_context = sdl2::ttf::init()?;
    let texture_creator = canvas.texture_creator();
    let theme = Theme::create(&ttf_context)?;
    let mut widget_factory = WidgetFactory::new(&theme, &texture_creator);

    let mut event_pump = sdl_context.event_pump()?;

    let mut app_state = ChangeDetector::new(AppState::default());
    let mut main_form_state = MainFormState::default();

    let mut main_form = MainForm::new(&mut widget_factory, *window_rect, app_state.click_count)?;

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
        if main_form_state.btn_cancel.was_clicked() {
            break 'running;
        }

        canvas.set_draw_color(Color::RGB(0x3c, 0x3f, 0x41));
        canvas.clear();

        let mut render_ctx = RenderContext::new(&mut canvas);
        let constraints = RenderConstraints::new(*window_rect);
        main_form.render(&main_form_state, constraints, &mut render_ctx)?;

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
