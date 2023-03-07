
use std::collections::HashMap;
use sdl2::{render::{Texture, TextureCreator}, ttf::Sdl2TtfContext};

use crate::{color::Color, gc2d::Gc2dResult};

pub type FontContext<'a> = sdl2::ttf::Sdl2TtfContext;
pub type FontStyle = sdl2::ttf::FontStyle;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Font {
    pub filename: String,
    pub point_size: u16,
}

pub struct FontSize {
  pub height: u32,
  pub width: u32,
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

    pub(crate) fn new_font(&mut self, ttf_context: &'ttf Sdl2TtfContext, font_key: Font) -> Gc2dResult<()> {
     
      let font = ttf_context.load_font(font_key.filename.clone(), font_key.point_size)?;
      
      self.fonts.insert(
        font_key, 
        Box::new(font),
      );

      Ok(())
    }

    pub(crate) fn get_texture(&mut self, texture_creator: &TextureCreator<sdl2::video::WindowContext>, font_key: &Font, texte: String, color: &Color) -> Option<Texture> {

      if let Some(detail) = self.fonts.get_mut(&font_key) {
        
        let font = detail.as_mut();

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

    pub(crate) fn get_font_height(&self, text: &str, font_key: &Font) -> Gc2dResult<u32> {
      if let Ok(size) = self.get_font_size(text, font_key) {
        Ok(size.height)
      } else {
        Err(String::from("Font size not found"))
      }
    }

    pub(crate) fn get_font_width(&self, text: &str, font_key: &Font) -> Gc2dResult<u32>  {
      if let Ok(size) = self.get_font_size(text, font_key) {
        Ok(size.width)
      } else {
        Err(String::from("Font size not found"))
      }
    }

    pub(crate) fn get_font_size(&self, text: &str, font_key: &Font) -> Gc2dResult<FontSize>  {
      if let Some(detail) = self.fonts.get(&font_key) {
        let font = detail.as_ref();
        if let Ok(size) = font.size_of(text) {
          return Ok(FontSize { height: size.1, width: size.0 });
        } else {
          return Err(String::from("Font size not found"));
        }
      }
      Err(String::from("Font not found"))
    }

}