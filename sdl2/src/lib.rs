use amulet_core::{mouse, GlobalEvent, VuiError};
use sdl2::event::Event as SdlEvent;
use sdl2::render::TextureValueError;
use sdl2::ttf::FontError;
use sdl2::EventPump;
use std::fmt::Display;

pub mod render;
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
        SdlEvent::MouseMotion { x, y, .. } => Some(GlobalEvent::MouseMotion((x, y).into())),
        SdlEvent::MouseButtonUp {
            x, y, mouse_btn, ..
        } => map_mouse_button(mouse_btn).map(|btn| GlobalEvent::MouseButtonUp(btn, (x, y).into())),
        SdlEvent::MouseButtonDown {
            x, y, mouse_btn, ..
        } => {
            map_mouse_button(mouse_btn).map(|btn| GlobalEvent::MouseButtonDown(btn, (x, y).into()))
        }
        sdl_event => return Some(Event::Sdl(sdl_event)),
    };

    amu_event.map(Event::Amulet)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Amulet(GlobalEvent),
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
    let pre_iter = [Event::Amulet(GlobalEvent::LoopStart)].into_iter();
    let event_iter = event_pump.poll_iter();
    EventIter {
        pre_iter,
        event_iter,
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
