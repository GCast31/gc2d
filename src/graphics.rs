
use sdl2::{render::{Canvas, CanvasBuilder}, Sdl};

use crate::{window::Window, context::Context};


pub struct Graphics {
    canvas: Option<Canvas<sdl2::video::Window>>,
}

impl Default for Graphics {
    fn default() -> Self {
        Graphics { 
            canvas: None, 
        }
    }
}

impl Graphics {
    pub fn new(window: &Window, ctx: &Context) -> Self {
        let video_subsystem = ctx.context.video().unwrap();

        /* Create the window */
        let window = video_subsystem
            .window("title", window.get_width() as u32, window.get_height() as u32)
            .position_centered()
            .build()
            .unwrap();

        // Canvas
        let mut canvas= window.into_canvas().build().unwrap();
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.clear();
        canvas.present();

        Graphics { 
            canvas: Some(canvas),  
        }
    }

    /***********************************************************
     * begin_draw()
     *
     * @brief : Prepare to drawing, call before drawing
     **********************************************************/
    pub(crate) fn begin_draw(&mut self) {
        //self.set_color(self.background_color);
        if let Some(canvas) = &mut self.canvas {
            canvas.clear();
        }
        //self.set_color_to_default();
    }

    /***********************************************************
     * end_draw()
     *
     * @brief : Call after drawing
     **********************************************************/
     pub(crate) fn end_draw(&mut self) {
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
        }
    }
}