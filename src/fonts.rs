
use std::{collections::HashMap, rc::Rc, marker::PhantomData, cell::RefCell};
use sdl2::{render::{Texture, Canvas, TextureCreator}, video::Window, ttf::Sdl2TtfContext};

use crate::{color::Color, graphics::{Graphics, FontsCreator}};

pub type FontContext<'a> = sdl2::ttf::Sdl2TtfContext;
pub type FontStyle = sdl2::ttf::FontStyle;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Font {
    pub filename: String,
    pub point_size: u16,
}

pub struct FontsManager<'ttf, 'rwops: 'ttf> {
    fonts: HashMap<Font, Box<sdl2::ttf::Font<'ttf, 'rwops>>>,
}

impl<'ttf, 'rwops> FontsManager<'ttf, 'rwops> {
    pub(crate) fn new() -> Self {
        Self {
          fonts: HashMap::new(),
        }
    }

    pub(crate) fn new_font(&mut self, ttf_context: &'ttf Sdl2TtfContext, font_key: Font) -> Result<(), String>{
     
      let font = ttf_context.load_font(font_key.filename.clone(), font_key.point_size)?;
      
      self.fonts.insert(
        font_key, 
        Box::new(font),
      );

      Ok(())
    }

    pub(crate) fn get_texture(&mut self, texture_creator: &TextureCreator<sdl2::video::WindowContext>, font_key: &Font, texte: String, color: &Color) -> Option<Texture> {

      if let Some(detail) = self.fonts.get_mut(&font_key) {
        
        let mut font = detail.as_mut();

        font.set_style(FontStyle::NORMAL);

        if let Ok(surface) = font
            .render(texte.as_str())
            .blended(Color::to_sdl_color(color)) {

          if let Ok(texture) = 
              texture_creator
              .create_texture_from_surface(&surface) {
            return Some(texture);
          }
        }  

      }

      return Option::None;
    }

}