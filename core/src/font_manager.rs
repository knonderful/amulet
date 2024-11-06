use crate::resource_manager::{ResourceLoader, ResourceManager};
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::PathBuf;

pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: PathBuf,
    pub size: u16,
}

impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        self.load_font(&details.path, details.size)
    }
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}
