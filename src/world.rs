// here, the object describing the world

use crate::{
    common_structs::{
        RGBAColor,
        Coord, Angle, Palette,
    },
    rgba_canvas::RGBACanvas,
    agent::Agent,
    shape::Shape,
    linear_texture::{
        LinearTexture,
        TransType,
        TextType,
    },
};


pub struct World {
    pub width: f32, //world width
    pub height: f32, //world height
    pub static_background: RGBACanvas, //contains all static objects pre-rendered(?) or just a backdrop
    pub shapes: Vec<Shape>,
    // pub agent: Agent,
    pub is_updated: bool,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut new_world: World = World {
            width: width as f32,
            height: height as f32,
            static_background: RGBACanvas::new(width, height),
            shapes: Vec::new(),
            is_updated: false,
        };

        new_world.add_shapes();
        new_world.create_static_background();

        return new_world;
    }

/////////////////////////////////////////////////////////

    /* pub fn collide_agent(&mut self) {
        let distance_from_top_wall: f32 = self.shapes[2].elements[1].new_shifted(self.shapes[2].anchor).get_distance(self.agent.center).unwrap_or_default();
        println!("distance to top wall is: {:.1}", distance_from_top_wall);
    } */

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
            String::from("Pentagon shape"),
            100.0,
            5,
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
        self.shapes[1].shift(Coord::new(150.0, 200.0));
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

        self.shapes.push(Shape::new_box(
            String::from("Middle wall"),
            20.0,
            400.0,
            LinearTexture::new(
                RGBAColor::new_p(Palette::LightGrey),
                RGBAColor::new_p(Palette::White),
                10.0,
                TransType::Lin,
                RGBAColor::new_p(Palette::DarkGrey),
                40.0,
                0.0,
                TextType::Step,
                0.25,
            ),
        ).unwrap());
        self.shapes[6].shift(Coord::new(350.0, 320.0));

    }
    
}