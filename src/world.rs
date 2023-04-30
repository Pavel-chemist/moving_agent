// here, the object describing the world

use crate::{common_structs::{RGBACanvas, RGBAColor, Line, Ellipse, Coord}};


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
            static_background: RGBACanvas::new(width, height),
            lines: Vec::new(),
            ellipses: Vec::new(),
            is_updated: true,
        };
    }

    pub fn get_rendered_view(&self) -> RGBACanvas {    
        let mut rendered_scene = self.static_background.copy();

        for i in 0..self.ellipses.len() {
            rendered_scene.draw_ellipse(&self.ellipses[i]);
        }

        return rendered_scene;
    }

/////////////////////////////////////////////////////////
    
    
}