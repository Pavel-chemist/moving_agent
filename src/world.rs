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
    // pub lines: Vec<LineSeg>,
    // pub ellipses: Vec<Ellipse>,
    // pub polygons: Vec<Polygon>,
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
            // lines: Vec::new(),
            // ellipses: Vec::new(),
            // polygons: Vec::new(),
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
        self.shapes.push(Shape::new_box(
            String::from("Box shape"),
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
        ).unwrap());
        self.shapes[0].shift(Coord::new(500.0, 300.0));
        self.shapes[0].rotate(Angle::new_deg(11.0));
        
        self.shapes.push(Shape::new_regular_polygon(
            String::from("Triangle shape"),
            100.0,
            3,
            LinearTexture::new(
                RGBAColor::new_p(Palette::Red),
                RGBAColor::new_p(Palette::Yellow),
                10.0,
                TransType::Quad,
                RGBAColor::new_p(Palette::DarkRed),
                20.0,
                0.0,
                TextType::Step,
                0.3333,
            ),
        ).unwrap());
        self.shapes[1].shift(Coord::new(100.0, 100.0));
        self.shapes[1].rotate(Angle::new_deg(-11.0));


        self.shapes.push(Shape::new_box(
            String::from("Top wall"),
            800.0,
            20.0,
            LinearTexture::new(
                RGBAColor::new_p(Palette::Yellow),
                RGBAColor::new_p(Palette::White),
                10.0,
                TransType::Lin,
                RGBAColor::new_p(Palette::Orange),
                50.0,
                0.0,
                TextType::Step,
                0.2,
            ),
        ).unwrap());
        self.shapes[2].shift(Coord::new(400.0, 10.0));

        self.shapes.push(Shape::new_box(
            String::from("Bottom wall"),
            800.0,
            20.0,
            LinearTexture::new(
                RGBAColor::new_p(Palette::Cyan),
                RGBAColor::new_p(Palette::White),
                10.0,
                TransType::Lin,
                RGBAColor::new_p(Palette::Blue),
                50.0,
                0.0,
                TextType::Step,
                0.2,
            ),
        ).unwrap());
        self.shapes[3].shift(Coord::new(400.0, 510.0));

        self.shapes.push(Shape::new_box(
            String::from("Left wall"),
            20.0,
            520.0,
            LinearTexture::new(
                RGBAColor::new_p(Palette::Orange),
                RGBAColor::new_p(Palette::White),
                10.0,
                TransType::Lin,
                RGBAColor::new_p(Palette::Red),
                50.0,
                0.0,
                TextType::Step,
                0.2,
            ),
        ).unwrap());
        self.shapes[4].shift(Coord::new(10.0, 260.0));

        self.shapes.push(Shape::new_box(
            String::from("Left wall"),
            20.0,
            520.0,
            LinearTexture::new(
                RGBAColor::new_p(Palette::Green),
                RGBAColor::new_p(Palette::White),
                10.0,
                TransType::Lin,
                RGBAColor::new_p(Palette::DarkGreen),
                50.0,
                0.0,
                TextType::Step,
                0.2,
            ),
        ).unwrap());
        self.shapes[5].shift(Coord::new(770.0, 260.0));

    }
    
}