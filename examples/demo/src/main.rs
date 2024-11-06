use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use std::path::PathBuf;
use std::rc::Rc;
use vui_core::component::mouse_aware::{MouseAware, MouseAwareCtx};
use vui_core::component::{ComponentEvent, HandleEvent, MouseButton, Pos, PositionalComponent, Render, Text, View};
use vui_core::font_manager::{FontDetails, FontManager};
use vui_core::generator::Generator;
use vui_core::math::Convert;
use vui_core::render::{RenderConstraints, RenderDestination};
use vui_core::util::TupleExtend as _;

type MyLabel<'ttf> = Pos<Text<(Rc<Font<'ttf, 'static>>, Color)>>;

struct MyGui<'ttf> {
    label_1: MyLabel<'ttf>,
    label_2: MyLabel<'ttf>,
}

impl<'ttf> Generator for MyGui<'ttf> {
    type State = usize;
    type Item = dyn Render + 'ttf;

    fn next(&self, iter_state: &mut Self::State) -> Option<&Self::Item> {
        let out = match iter_state {
            0 => &self.label_1,
            1 => &self.label_2,
            _ => return None,
        };
        *iter_state += 1;
        Some(out)
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

    let labels: Vec<MyLabel> = (0..20i32)
        .map(|i| {
            Pos::new(
                (20 + 40 * i, 20 + 8 * i).into(),
                Text::new(
                    "Hello world".into(),
                    ().extend(font.clone()).extend(Color::RGB(
                        255,
                        ((i * 50) % 256).convert_or(0),
                        0,
                    )),
                ),
            )
        })
        .collect();

    let view = View::new(labels);

    let mut labels_2: Vec<Box<MyLabel>> = (0..20i32)
        .map(|i| {
            Pos::new(
                (20 + 40 * i, 120 + 8 * i).into(),
                Text::new(
                    "Hello world".into(),
                    ().extend(font.clone()).extend(Color::RGB(
                        255,
                        0,
                        ((i * 50) % 256).convert_or(0),
                    )),
                ),
            )
        })
        .map(Box::new)
        .collect();

    let view_2 = View::new(labels_2.as_mut_slice());

    let boxed = Box::new(Pos::new(
        (220, 220).into(),
        Text::new(
            "LABEL IN A BOX".into(),
            ().extend(font.clone()).extend(Color::RGB(255, 0, 0)),
        ),
    ));

    let veccy: Vec<Box<dyn PositionalComponent>> = vec![boxed];
    let view_3 = View::new(veccy);

    let mut dyned = Pos::new(
        (240, 240).into(),
        Text::new(
            "LABEL BEHIND A DYN ARRAY".into(),
            ().extend(font.clone()).extend(Color::RGB(255, 0, 0)),
        ),
    );

    let mut arr: [&mut dyn PositionalComponent; 1] = [&mut dyned];
    let view_4 = View::new(arr.as_mut_slice());

    let my_gui = MyGui {
        label_1: Pos::new(
            (250, 250).into(),
            Text::new(
                "MYGUI LABEL 1".into(),
                ().extend(font.clone()).extend(Color::RGB(255, 0, 0)),
            ),
        ),
        label_2: Pos::new(
            (250, 270).into(),
            Text::new(
                "MYGUI LABEL 2".into(),
                ().extend(font.clone()).extend(Color::RGB(255, 0, 0)),
            ),
        ),
    };
    let view_5 = View::new(my_gui);

    let mut ma_ctx = MouseAwareCtx::default();
    let mut click_count = 0;

    'running: loop {
        let mut mouse_aware = Pos::new(
            (100, 300).into(),
            MouseAware::new(
                &mut ma_ctx,
                Text::new(
                    "MOUSE AWARE".into(),
                    ().extend(font.clone()).extend(Color::RGB(255, 0, 0)),
                ),
            ),
        );

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    // (mouse_x, mouse_y) = (x,y);
                    mouse_aware.handle_event(ComponentEvent::MouseMotion((x, y).into()))?;
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let Ok(btn) = mouse_btn.try_into() else {
                        continue;
                    };
                    mouse_aware.handle_event(ComponentEvent::MouseButtonUp(btn, (x, y).into()))?;
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let Ok(btn) = mouse_btn.try_into() else {
                        continue;
                    };
                    mouse_aware
                        .handle_event(ComponentEvent::MouseButtonDown(btn, (x, y).into()))?;
                }
                _ => {}
            }
        }

        // view_4.handle_event(ComponentEvent::MousePosition((mouse_x, mouse_y).into()))?;
        // view_4.handle_event(ComponentEvent::MouseClickState(mouse_clicked))?;
        // view_4.handle_event(ComponentEvent::Finalize)?;

        // mouse_aware.handle_event(ComponentEvent::Finalize)?;

        canvas.set_draw_color(Color::RGB(24, 24, 24));
        canvas.clear();

        let mut render_dest = RenderDestination::new(&texture_creator, &mut canvas);
        let constraints = RenderConstraints::new(Rect::new(0, 0, 800, 600));
        view.render((&mut render_dest, constraints.clone()))?;
        view_2.render((&mut render_dest, constraints.clone()))?;

        view_3.render((&mut render_dest, constraints.clone()))?;
        view_4.render((&mut render_dest, constraints.clone()))?;
        view_5.render((&mut render_dest, constraints.clone()))?;

        mouse_aware.render((&mut render_dest, constraints.clone()))?;

        let click_count_label = Pos::new(
            (10, 400).into(),
            Text::new(
                format!("Click count: {click_count}").into(),
                ().extend(font.clone()).extend(Color::RGB(255, 0, 0)),
            ),
        );
        click_count_label.render((&mut render_dest, constraints.clone()))?;

        if ma_ctx.click_started(MouseButton::Left) {
            println!("CLICK STARTED");
        }
        if ma_ctx.click_completed(MouseButton::Left) {
            println!("CLICK COMPLETED");
            click_count += 1;
        }

        canvas.present();
    }

    Ok(())
}
