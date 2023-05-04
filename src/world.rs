// here, the object describing the world

use crate::{
    common_structs::{
        RGBACanvas,
        RGBAColor,
        Coord,
    },
    ellipse::Ellipse,
    line::Line,
    polygon::{
        Polygon,
        PType,
    },
};


pub struct World {
    pub width: f64, //world width
    pub height: f64, //world height
    static_background: RGBACanvas, //contains all static objects pre-rendered
    // pub lines: Vec<Line>,
    pub ellipses: Vec<Ellipse>,
    // pub sprites: Vec<Sprite>,
    pub shapes: Vec<Polygon>,
    pub is_updated: bool,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut new_world: World = World {
            width: width as f64,
            height: height as f64,
            static_background: RGBACanvas::new(width, height),
            // static_background: Self::create_static_bacground(width, height),
            // lines: Vec::new(),
            ellipses: Vec::new(),
            shapes: Vec::new(),
            is_updated: false,
        };

        new_world.add_shapes();
        new_world.create_static_bacground();

        return new_world;
    }

    pub fn get_rendered_view(&self) -> RGBACanvas {
        let mut rendered_scene = self.static_background.clone();

        for i in 0..self.shapes.len() {
            self.shapes[i].draw(&mut rendered_scene);
        }

        for i in 0..self.ellipses.len() {
            self.ellipses[i].draw_ellipse_raster(&mut rendered_scene, false, 20.0);
        }

        return rendered_scene;
    }

/////////////////////////////////////////////////////////
    fn create_static_bacground(&mut self) {
        let mut background: RGBACanvas = RGBACanvas::new_f(self.width, self.height);

        /* for i in 0..self.shapes.len() {
            self.shapes[i].draw(&mut background);
        } */
        

        self.static_background = background;
        self.is_updated = true;
    }

    fn add_shapes(&mut self) {
        for i in 0..6 {
            self.shapes.push(
                Polygon::new(PType::Regular { 
                    n: i + 3,
                    r: 50.0,
                    pivot: Coord::new(75.0 + 125.0 * i as f64, 100.0),
                    color: RGBAColor::new_rgb(255, 255, 255),
                }
            ).unwrap());
        }
    }
    
}