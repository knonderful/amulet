use amulet_core::component::ComponentEvent;
use amulet_core::render::RenderConstraints;
use amulet_core::{mouse, VuiError, VuiResult};
use sdl2::event::Event as SdlEvent;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::{TextureCreator, TextureValueError, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::FontError;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::fmt::Display;

pub mod temp_components;

fn map_mouse_button(value: sdl2::mouse::MouseButton) -> Option<mouse::Button> {
    use sdl2::mouse::MouseButton as MB;
    let out = match value {
        MB::Unknown | MB::X1 | MB::X2 => return None,
        MB::Left => mouse::Button::Left,
        MB::Middle => mouse::Button::Middle,
        MB::Right => mouse::Button::Right,
    };
    Some(out)
}

fn map_event(sdl_event: SdlEvent) -> Option<Event> {
    let amu_event = match sdl_event {
        SdlEvent::MouseMotion { x, y, .. } => Some(ComponentEvent::MouseMotion((x, y).into())),
        SdlEvent::MouseButtonUp {
            x, y, mouse_btn, ..
        } => {
            map_mouse_button(mouse_btn).map(|btn| ComponentEvent::MouseButtonUp(btn, (x, y).into()))
        }
        SdlEvent::MouseButtonDown {
            x, y, mouse_btn, ..
        } => map_mouse_button(mouse_btn)
            .map(|btn| ComponentEvent::MouseButtonDown(btn, (x, y).into())),
        sdl_event => return Some(Event::Sdl(sdl_event)),
    };

    amu_event.map(Event::Amulet)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Amulet(ComponentEvent),
    Sdl(SdlEvent),
}

struct EventIter<'a> {
    pre_iter: std::array::IntoIter<Event, 1>,
    event_iter: sdl2::event::EventPollIterator<'a>,
}

impl Iterator for EventIter<'_> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.pre_iter
            .next()
            .or_else(|| self.event_iter.next().and_then(map_event))
    }
}

pub fn event_iterator(event_pump: &mut EventPump) -> impl Iterator<Item = Event> + '_ {
    let pre_iter = [Event::Amulet(ComponentEvent::LoopStart)].into_iter();
    let event_iter = event_pump.poll_iter();
    EventIter {
        pre_iter,
        event_iter,
    }
}

pub struct RenderContext<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    canvas: &'a mut WindowCanvas,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &'a mut WindowCanvas,
    ) -> Self {
        Self {
            texture_creator,
            canvas,
        }
    }

    fn blit_surface(&mut self, constraints: RenderConstraints, surface: &Surface) -> VuiResult<()> {
        let texture = self
            .texture_creator
            .create_texture_from_surface(surface)
            .map_err(map_error)?;
        let (x, y) = {
            let rect = constraints.clip_rect;
            (rect.min.x, rect.min.y)
        };

        let (w, h) = surface.size();
        // TODO: Clipping when the clip_rect is smaller than the surface... can either use set_clip_rect() or do it manually...
        //       Also note that for textures we wouldn't know the size... how do we handle that and is that relevant to this here...?
        self.canvas.copy(&texture, None, SdlRect::new(x, y, w, h))?;
        Ok(())
    }
}

trait TypeName {
    fn type_name() -> &'static str;
}

impl TypeName for TextureValueError {
    fn type_name() -> &'static str {
        "TextureValueError"
    }
}

impl TypeName for FontError {
    fn type_name() -> &'static str {
        "FontError"
    }
}

fn map_error<T>(e: T) -> VuiError
where
    T: Display + TypeName,
{
    VuiError::new(format!("{}: {}", T::type_name(), e))
}
