// here, the object describing the world

use crate::{common_structs::{RGBACanvas, RGBAColor}, ellipse::Ellipse, line::Line};


pub struct World {
    pub width: f64, //world width
    pub height: f64, //world height
    static_background: RGBACanvas, //contains all static objects pre-rendered
    pub lines: Vec<Line>,
    pub ellipses: Vec<Ellipse>,
    // pub sprites: Vec<Sprite>,
    pub is_updated: bool,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        return World {
            width: width as f64,
            height: height as f64,
            // static_background: RGBACanvas::new_black(width, height),
            static_background: RGBACanvas::new_color(width, height, RGBAColor::new_rgb(0, 0, 127)),
            lines: Vec::new(),
            ellipses: Vec::new(),
            is_updated: true,
        };
    }

    pub fn get_rendered_view(&self) -> RGBACanvas {    
        // let mut rendered_scene = self.static_background.copy();
        let mut rendered_scene = self.static_background.clone();

        for i in 0..self.ellipses.len() {
            self.ellipses[i].draw_ellipse_raster(&mut rendered_scene, false, 20.0);
        }

        return rendered_scene;
    }

/////////////////////////////////////////////////////////
    
    
}