
use std::collections::HashMap;
use sdl2::{render::{Texture, Canvas}, video::Window};

use crate::color::Color;

pub type FontContext<'a> = sdl2::ttf::Sdl2TtfContext;
pub type Font<'ttf, 'rwops> = sdl2::ttf::Font<'ttf, 'rwops>;
pub type FontStyle = sdl2::ttf::FontStyle;

#[derive(Hash, PartialEq, Eq, Debug)]
struct FontKey {
    filename: String,
    point_size: u16,
}
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct FontDetail {
  filename: String,
  point_size: u16,
  style: FontStyle,
}

pub struct FontsManager<'ttf, 'rwops> {
    _texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    fonts: HashMap<FontKey, Font<'ttf, 'rwops>>,
    context: FontContext<'ttf>,
}

impl<'ttf, 'rwops> FontsManager<'ttf, 'rwops> {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        Self {
          _texture_creator: canvas.texture_creator(), 
          fonts: HashMap::new(),
          context: sdl2::ttf::init().unwrap(),
        }
    }

    pub fn new_font(&'ttf mut self, filename: String, point_size: u16) -> Result<FontDetail, String>{
      let font_detail = FontDetail{
        filename: filename.to_string(),
        point_size,
        style: FontStyle::NORMAL,
      };
      let font_result = self.context.load_font(filename.clone(), point_size);
      match font_result {
          Ok(font ) => {
            let my_font = font;
            self.fonts.insert(FontKey {
              filename: filename.to_string(),
              point_size,
            }, my_font);
            return Ok(font_detail);
          },
          Err(e) => { return Err(e); }
      };
    }

    pub(crate) fn get_texture(&mut self, font_detail: &FontDetail, texte: String, color: &Color) -> Option<Texture> {

      let font_key = FontKey{
        filename: font_detail.filename.clone(),
        point_size: font_detail.point_size,
      };

      if self.fonts.contains_key(&font_key) {
        let font = self.fonts.get_mut(&font_key).unwrap();
        font.set_style(font_detail.style);

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