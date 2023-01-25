use sdl2::image::LoadTexture;
use std::collections::HashMap;

/*================================================================
 *                         _ I M A G E
 *================================================================*/
 pub trait ImageDescriptions {
    fn get_quad(&self) -> Option<Quad>;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
}

#[derive(Clone)]
pub(crate) enum ImageType {
    ImageFromFile(String),
    FromTexture,
}

 pub struct Image {
    width: f32,
    height: f32,
    pub(crate) texture: sdl2::render::Texture,
}

impl ImageDescriptions for Image {

    fn get_quad(&self) -> Option<Quad> {
        Option::None
    }
    
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }
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
pub struct Quad {
    filename: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Quad {
    pub fn new(filename: String, x: f32, y: f32, width: f32, height: f32) -> Quad {
        Quad { filename, x, y, width, height }
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl ImageDescriptions for Quad {
    fn get_quad(&self) -> Option<Quad> {
        Some(Quad {
            filename: self.filename.clone(),
            height: self.height,
            width: self.width,
            x: self.x,
            y: self.y,
        })
    }
    fn get_width(&self) -> f32 {
        self.width
    }
    fn get_height(&self) -> f32 {
        self.height
    }
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
    pub(crate) fn new_image(&mut self, filename: &str) -> Result<(), String> {

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
}