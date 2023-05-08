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
};


pub struct World {
    pub width: f64, //world width
    pub height: f64, //world height
    static_background: RGBACanvas, //contains all static objects pre-rendered
    pub lines: Vec<Line>,
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
            lines: Vec::new(),
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

        for i in 0..self.lines.len() {
            self.lines[i].draw(&mut rendered_scene);
        }

        for i in 0..self.shapes.len() {
            self.shapes[i].draw(&mut rendered_scene);
        }

        let num_lines: usize = self.lines.len();
        let num_shapes: usize = self.shapes.len();
        let mut point: Option<Coord>;

        for i in 0..self.shapes[num_shapes-1].sides.len() {
            point = self.shapes[num_shapes-1].sides[i].intersection(&self.lines[num_lines -1]);

            match point {
                Some(int_pt) => {
                    rendered_scene.put_square(
                        int_pt.get_x_i(),
                        int_pt.get_y_i(),
                        7,
                        RGBAColor::new_rgb(255, 127, 63)
                    );

                    rendered_scene.put_disc(
                        int_pt.get_x_i(),
                        int_pt.get_y_i(),
                        5,
                        RGBAColor::new_rgb(63, 127, 255)
                    );
                }
                None => {}
            }
        }

        for i in 0..self.ellipses.len() {
            self.ellipses[i].draw_ellipse_raster(&mut rendered_scene, false, 20.0);
        }

        return rendered_scene;
    }

/////////////////////////////////////////////////////////
    fn create_static_bacground(&mut self) {
        let background: RGBACanvas = RGBACanvas::new_f(self.width, self.height);

        self.static_background = background;
        self.is_updated = true;
    }

    fn add_shapes(&mut self) {
        self.lines.push(Line::new(Coord::new(0.0, 100.0), Coord::new(200.0, 100.0), RGBAColor::new_rgb(255, 0, 0)));

        self.lines.push(Line::new(Coord::new(100.0, 0.0), Coord::new(100.0, 200.0), RGBAColor::new_rgb(255, 0, 0)));

        self.lines.push(Line::new(Coord::new(100.0, 290.0), Coord::new(600.0, 310.0), RGBAColor::new_rgb(0, 255, 0)));


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

        self.shapes[4].move_pivot(Coord::new(30.0, 10.0));

        self.shapes.push(Polygon::new(PType::Rectangle { 
                length: 200.0,
                width: 100.0,
                pivot: Coord::new(400.0, 300.0),
                color: RGBAColor::new_rgb(255, 255, 0),
            }
        ).unwrap());

        self.shapes[6].move_pivot(Coord::new(50.0, 20.0));
        self.shapes[6].rotate(Angle::new_rad(1.0));
    }
    
}