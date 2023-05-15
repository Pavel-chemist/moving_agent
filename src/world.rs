// here, the object describing the world

use crate::{
    common_structs::{
        RGBAColor,
        Coord, Angle, Palette,
    },
    ellipse::Ellipse,
    line_seg::LineSeg,
    polygon::{
        Polygon,
        PType,
    },
    rgba_canvas::RGBACanvas,
    agent::Agent, shape::Shape, linear_texture::{LinearTexture, TransType, TextType},
};


pub struct World {
    pub width: f32, //world width
    pub height: f32, //world height
    pub static_background: RGBACanvas, //contains all static objects pre-rendered
    pub lines: Vec<LineSeg>,
    pub ellipses: Vec<Ellipse>,
    pub polygons: Vec<Polygon>,
    pub shapes: Vec<Shape>,
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
            polygons: Vec::new(),
            shapes: Vec::new(),
            agent: Agent::new(
                // Coord::new_i(width / 2, height / 2),
                Coord::new_i(200, 310),
                Angle::new_deg(0.0),
                Angle::new_deg(120.0),
                500.0,
            ),
            is_updated: false,
        };

        new_world.add_shapes();
        new_world.create_static_background();

        return new_world;
    }

/////////////////////////////////////////////////////////
    fn create_static_background(&mut self) {
        let mut background: RGBACanvas = RGBACanvas::new_f(self.width, self.height);
        let grid_color: RGBAColor = RGBAColor::new_p(Palette::DarkGrey);
        for j in 0..background.height {
            for i in 0..background.width {
                if j % 100 == 0 || i % 100 == 0 {
                    background.put_pixel(i, j, grid_color);
                }
            }
        }

        self.static_background = background;
        self.is_updated = true;
    }

    fn add_shapes(&mut self) {
        /* for i in 0..6 {
            self.polygons.push(
                Polygon::new(PType::Regular { 
                    n: i + 3,
                    r: 50.0,
                    pivot: Coord::new(75.0 + 125.0 * i as f32, 100.0),
                    color: RGBAColor::new_rand(),
                }
            ).unwrap());
        }

        self.polygons[4].move_pivot(Coord::new(30.0, 10.0));

        self.polygons.push(Polygon::new(
            PType::Rectangle { 
                length: 100.0,
                width: 100.0,
                pivot: Coord::new(400.0, 300.0),
                color: RGBAColor::new_p(Palette::Orange),
            }
        ).unwrap());

        self.polygons.push(Polygon::new(
            PType::Rectangle { 
                length: 100.0,
                width: 100.0,
                pivot: Coord::new(100.0, 200.0),
                color: RGBAColor::new_p(Palette::Yellow),
            }
        ).unwrap());

        self.polygons.push(Polygon::new(
            PType::Rectangle { 
                length: 100.0,
                width: 100.0,
                pivot: Coord::new(169.0, 75.0),
                color: RGBAColor::new_p(Palette::Grass),
            }
        ).unwrap()); */

        self.polygons.push(Polygon::new(PType::Rectangle { 
                length: 700.0,
                width: 10.0,
                pivot: Coord::new(400.0, 500.0),
                color: RGBAColor::new_rgb(255, 255, 0),
            }
        ).unwrap());

        self.polygons.push(Polygon::new(PType::Rectangle { 
                length: 50.0,
                width: 50.0,
                pivot: Coord::new(100.0, 450.0),
                color: RGBAColor::new_rgb(255, 127, 0),
            }
        ).unwrap());

        self.shapes.push(Shape::new_box(
            String::from("Shape of new type"),
            100.0,
            200.0,
            LinearTexture::new(
                RGBAColor::new_p(Palette::Grass),
                RGBAColor::new_p(Palette::White),
                10.0,
                TransType::Lin,
                RGBAColor::new_p(Palette::DarkGreen),
                70.0,
                0.0,
                TextType::Step,
                0.2,
            ),
        ));

        self.shapes[0].shift(Coord::new(500.0, 300.0))
        
    }
    
}