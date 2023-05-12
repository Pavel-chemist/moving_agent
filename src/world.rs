// here, the object describing the world

use crate::{
    common_structs::{
        RGBAColor,
        Coord, Angle,
    },
    ellipse::Ellipse,
    line::Line,
    polygon::{
        Polygon,
        PType,
    },
    rgba_canvas::RGBACanvas,
    agent::Agent,
};


pub struct World {
    pub width: f32, //world width
    pub height: f32, //world height
    pub static_background: RGBACanvas, //contains all static objects pre-rendered
    pub lines: Vec<Line>,
    pub ellipses: Vec<Ellipse>,
    pub shapes: Vec<Polygon>,
    pub agent: Agent,
    pub is_updated: bool,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut new_world: World = World {
            width: width as f32,
            height: height as f32,
            static_background: RGBACanvas::new(width, height),
            lines: Vec::new(),
            ellipses: Vec::new(),
            shapes: Vec::new(),
            agent: Agent::new(
                Coord::new_i(width / 2, height / 2),
                Angle::new(),
                Angle::new_deg(90.0),
                512.0,
                // 1024.0,
            ),
            is_updated: false,
        };

        new_world.add_shapes();
        new_world.create_static_background();

        return new_world;
    }

/////////////////////////////////////////////////////////
    fn create_static_background(&mut self) {
        let background: RGBACanvas = RGBACanvas::new_f(self.width, self.height);

        self.static_background = background;
        self.is_updated = true;
    }

    fn add_shapes(&mut self) {
        for i in 0..6 {
            self.shapes.push(
                Polygon::new(PType::Regular { 
                    n: i + 3,
                    r: 50.0,
                    pivot: Coord::new(75.0 + 125.0 * i as f32, 100.0),
                    color: RGBAColor::new_rand(),
                }
            ).unwrap());
        }

        self.shapes[4].move_pivot(Coord::new(30.0, 10.0));

        self.shapes.push(Polygon::new(PType::Rectangle { 
                length: 700.0,
                width: 10.0,
                pivot: Coord::new(400.0, 500.0),
                color: RGBAColor::new_rgb(255, 255, 0),
            }
        ).unwrap());

        self.shapes.push(Polygon::new(PType::Rectangle { 
                length: 50.0,
                width: 50.0,
                pivot: Coord::new(100.0, 450.0),
                color: RGBAColor::new_rgb(255, 127, 0),
            }
        ).unwrap());
    }
    
}