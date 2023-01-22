
use sdl2::{render::{Canvas, TextureCreator}, video::WindowContext};
use crate::{context::Context, color::Color, fonts::{FontsManager, Font}, image::{ImageType, ImageDescriptions, Image}};


pub type FontsCreator = TextureCreator<WindowContext>;

#[allow(dead_code)]
pub enum DrawMode {
    Fill,
    Line,
}

pub struct Graphics {
    pub(crate) canvas: Canvas<sdl2::video::Window>,

    pub(crate) _new_fonts: Vec<Font>,

    actual_font: Option<Font>,

    actual_color: Color,
    background_color: Color,
    default_color: Color,

    actual_sx: f32,
    actual_sy: f32,
}

impl Graphics {
    //=======================================================================
    //                               GENERAL
    //=======================================================================
    pub(crate) fn new(ctx: &Context, window: &crate::window::Window) -> Self {
        let video_subsystem = ctx.context.video().unwrap();

        /* Create the window */
        let window = video_subsystem
            .window(window.title.as_str(), window.width as u32, window.height as u32)
            .position_centered()
            .build()
            .unwrap();

        // Canvas
        let mut canvas= window.into_canvas().build().unwrap();
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.clear();
        canvas.present();

        Graphics { 
            canvas,  
            actual_color: Color::WHITE,
            default_color: Color::WHITE,
            background_color: Color::BLACK,
            actual_font: None,
            actual_sx: 1.,
            actual_sy: 1.,

            _new_fonts: Vec::new(),
        }
    }

    pub fn get_canvas_ref_mut(&mut self) -> &mut Canvas<sdl2::video::Window> {
        &mut self.canvas
    }


    /***********************************************************
     * get_fonts_creator
     *
     * @Brief : Create a texture for fonts
     */
    pub fn get_fonts_creator(&self) -> FontsCreator {
        self.canvas.texture_creator() as FontsCreator
    }

    /***********************************************************
     * begin_draw()
     *
     * @brief : Prepare to drawing, call before drawing
     **********************************************************/
    pub(crate) fn begin_draw(&mut self) {
        self.set_color(self.background_color);
        self.canvas.clear();
        self.apply_default_color();
    }

    /***********************************************************
     * end_draw()
     *
     * @brief : Call after drawing
     **********************************************************/
     pub(crate) fn end_draw(&mut self) {
        self.canvas.present();
    }

    //=======================================================================
    //                             Color
    //=======================================================================
    pub fn apply_default_color(&mut self) {
        self.actual_color = self.default_color.clone();
    }

    pub fn set_color(&mut self, color: Color) {
        self.actual_color = color;
        self.canvas.set_draw_color(self.actual_color.to_sdl_color());
    }

    pub fn set_default_color(&mut self, color: Color) {
        self.default_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    //=======================================================================
    //                             PRIMITIVES
    //=======================================================================
    /***********************************************************
     * draw_line()
     *
     * @brief : Draw a line
     */
    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Option<Color>) {

        let actual_color = self.actual_color;

        if let Some(color) = color {
            self.set_color(color);
        }

        self.canvas
            .draw_line(
                sdl2::rect::Point::new(x1 as i32, y1 as i32),
                sdl2::rect::Point::new(x2 as i32, y2 as i32),
            )
            .unwrap();
        
        if let Some(_) = color {
            self.set_color(actual_color);
        }
    }

    /***********************************************************
     * rectangle()
     *
     * @brief : Draw a rectangle
     */
    pub fn rectangle(&mut self, mode: DrawMode, x: f32, y: f32, width: f32, height: f32, color: Option<Color>) {

        let actual_color = self.actual_color;

        if let Some(color) = color {
            self.set_color(color);
        }

        match mode {
            DrawMode::Fill => {
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new( x as i32, y as i32, width as u32, height as u32)) 
                    .unwrap();
        
            },
            DrawMode::Line => {
                self.canvas
                    .draw_rect(sdl2::rect::Rect::new( x as i32, y as i32, width as u32, height as u32)) 
                    .unwrap();
            },
        }

        if let Some(_) = color {
            self.set_color(actual_color);
        }
    }

    //=======================================================================
    //                             SCALE
    //=======================================================================
    pub fn set_scale(&mut self, sx: f32, sy: f32) {
        self.actual_sx = sx;
        self.actual_sy = sy;
    }

    //=======================================================================
    //                             IMAGES
    //=======================================================================
    pub(crate) fn draw_image(&mut self, image_type: ImageType, image: &Image, x: f32, y: f32, angle: f64, scale_x: f32, scale_y: f32, origin_x: f32, origin_y: f32) {
        let mut scalex = scale_x * self.actual_sx;
        let mut scaley = scale_y * self.actual_sy;

        let mut dst = sdl2::rect::Rect::new((x * self.actual_sx)as i32,(y * self.actual_sy) as i32, image.get_width() as u32, image.get_height() as u32);
        dst.h = ((dst.h as f32) * scalex) as i32;
        dst.w = ((dst.w as f32) * scaley) as i32;

        let mut src: Option<sdl2::rect::Rect> = Option::None;

        if let Some(q) = image.get_quad() {
            let rect = sdl2::rect::Rect::new((q.get_x() * self.actual_sx) as i32, (q.get_y() * self.actual_sy) as i32 , q.get_width() as u32, q.get_height() as u32);
            src = Some(rect);
            dst.h = ((rect.h as f32) * scalex) as i32;
            dst.w = ((rect.w as f32) * scaley) as i32;
        }

        let mut w_center = Option::None;
        if origin_x!=0. && origin_y!=0. {
            w_center = Some(sdl2::rect::Point::new(origin_x as i32, origin_y as i32));
        }

            let flip_h = 
                if scalex < 0. {
                    scalex *= -1.;
                    true
                } 
                else { 
                    false 
                };

            let flip_v = 
                if scaley < 0. {
                    scaley *= -1.;
                    true
                } 
                else { 
                    false 
                };

            self.canvas
                .copy_ex(
                    &image.texture, 
                    src, 
                    dst, 
                    angle, 
                    w_center, 
                    flip_h,
                    flip_v, 
                )
                .unwrap();
    }

    //=======================================================================
    //                             FONTS
    //=======================================================================
    pub fn new_font(&mut self, filename: &str, point_size: u16) -> Font {
        let font = Font {
                filename: String::from(filename),
                point_size
        };
        self._new_fonts.push(font.clone());
        font
    }

    pub fn set_font(&mut self, font: Option<Font>) {
        self.actual_font = font; 
    }

    pub fn print_full(&mut self, fonts: &mut FontsManager, text: String, x: f32, y: f32, angle: f64, scale_x: f32, scale_y: f32, origin_x: f32, origin_y: f32, color: Option<Color>) {
        // Only if font is set
        if let Some(font) = &self.actual_font {

            // Witch color ?
            let l_color = if let Some(color) = color {
                                    color 
                                } else {
                                    self.actual_color
                                };

           // Create an texture from Text
           let font_creator = self.get_fonts_creator();
           let texture = fonts.get_texture(&font_creator, &font, text, &l_color); 
         
           // Create an image from Texture
           let image = Image::from_texture(texture.unwrap());

           // Draw text
           self.draw_image(ImageType::FromTexture, &image, x, y, angle, scale_x, scale_y, origin_x, origin_y);
        }
    }
    
    pub fn print(&mut self, fonts: &mut FontsManager, text: String, x: f32, y: f32, color: Option<Color>) {
        self.print_full(fonts, text, x, y, 0f64, 1f32, 1f32, 0f32, 0f32, color);
    }

}