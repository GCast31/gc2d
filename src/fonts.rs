
use std::{collections::HashMap, rc::Rc, marker::PhantomData};
use sdl2::{render::{Texture, Canvas}, video::Window};

use crate::{color::Color, graphics::{Graphics, FontsCreator}};

pub type FontContext<'a> = sdl2::ttf::Sdl2TtfContext;
pub type Font<'ttf, 'rwops> = sdl2::ttf::Font<'ttf, 'rwops>;
pub type FontStyle = sdl2::ttf::FontStyle;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct FontKey {
    pub filename: String,
    pub point_size: u16,
}

pub struct FontsManager<'ttf, 'rwops> {
    _texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    fonts: HashMap<FontKey, Font<'ttf, 'rwops>>,
    _context: FontContext<'ttf>,
}

impl<'ttf, 'rwops> FontsManager<'ttf, 'rwops> {
    pub fn new(fonts_creator: FontsCreator) -> Self {

      let context = sdl2::ttf::init().unwrap();

        Self {
          _texture_creator: fonts_creator, 
          fonts: HashMap::new(),
          _context: context,
        }
    }

    pub fn new_font(&'static mut self, filename: &str, point_size: u16) -> Result<FontKey, String>{
     
      let font = self._context.load_font(filename.clone(), point_size)?;
      
      let font_key = FontKey { 
        filename: String::from(filename), 
        point_size 
      };

      let font_key_return = font_key.clone();

      self.fonts.insert(
        font_key, 
        font
      );

      Ok(font_key_return)
    }

    pub(crate) fn get_texture(&mut self, font_key: &FontKey, texte: String, color: &Color) -> Option<Texture> {

      if let Some(detail) = self.fonts.get_mut(&font_key) {
        
        let mut font = detail;

        font.set_style(FontStyle::NORMAL);

        if let Ok(surface) = font
            .render(texte.as_str())
            .blended(Color::to_sdl_color(color)) {

          if let Ok(texture) = self
              ._texture_creator
              .create_texture_from_surface(&surface) {
            return Some(texture);
          }
        }  

      }

      return Option::None;
    }

}