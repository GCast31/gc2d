use sdl2::image::LoadTexture;
use std::collections::HashMap;

use crate::gc2d::Gc2dResult;

/*================================================================
 *                         _ I M A G E
 *================================================================*/
pub(crate) enum ImageType<'a> {
    ImageFromFile(String, Option<Quad>),
    FromTexture(&'a Image),
}

 pub struct Image {
    pub width: f32,
    pub height: f32,
    pub(crate) texture: sdl2::render::Texture,
}

impl Image {
    pub(crate) fn from_texture(texture: sdl2::render::Texture) -> Self {
        let height = texture.query().height as f32;
        let width = texture.query().width as f32;
        Self {
            texture,
            height,
            width,
        }
    }
}

/*
 * Quad : A part of an image
 */
#[derive(Clone, Copy)]
pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

//=======================================================================
//                            Images MANAGER
//=======================================================================
pub(crate) struct ImagesManager {
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    images: HashMap<String, Image>,
}

#[allow(dead_code)]
impl ImagesManager {
    /*
     * new()
     * 
     * @Brief : Create a new ImagesManager
     */
    pub(crate) fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> ImagesManager {
        ImagesManager {
            texture_creator,
            images: HashMap::new(),
        }
    }

    /*
     * new_image()
     * 
     * @Brief : Try to load a new image in the images manager
     */
    pub(crate) fn new_image(&mut self, filename: &str) -> Gc2dResult<()> {

        if let Some(_) = self.images.get_mut(&filename.to_string()) {
            return Ok(());
        }

        let texture_result = self.texture_creator.load_texture(filename.clone());
        let texture: sdl2::render::Texture;
        match texture_result {
            Err(e) => {
                return Err(e);
            }
            Ok(t) => {
                texture = t;
            }
        }

        let height = texture.query().height as f32;
        let width = texture.query().width as f32;

        let image = Image {
            width,
            height,
            texture,
        };

        self.images.insert(filename.to_string(), image);
        
        Ok(())
    
    }

    /*
     * get_image()
     * 
     * @Brief : Get a image from the image manager
     */
    pub fn get_image(&self, filename: &str) -> Option<&Image> {
        self.images.get(&filename.to_string())
    }

    /*
     * get_image_height()
     */
    pub fn get_image_height(&self, filename: &str) -> f32 {
        if let Some(image) = self.images.get(&filename.to_string()) {
            image.height
        } else {
            0.
        }
    }

    /*
     * get_image_width())
     */
    pub fn get_image_width(&self, filename: &str) -> f32 {
        if let Some(image) = self.images.get(&filename.to_string()) {
            image.width
        } else {
            0.
        }
    }
}