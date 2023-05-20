// here, the object describing the world

use crate::{
    common_structs::{
        RGBAColor,
        Coord, Angle, Palette,
    },
    rgba_canvas::RGBACanvas,
    shape::Shape,
    linear_texture::{
        LinearTexture,
        TransType,
        TextType,
    },
    vector_2d::Vector2D,
};


pub struct World {
    pub width: f32, //world width
    pub height: f32, //world height
    pub static_background: RGBACanvas, //contains all static objects pre-rendered(?) or just a backdrop
    pub shapes: Vec<Shape>,
    pub walls: Vec<Vector2D>,
    pub is_updated: bool,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut new_world: World = World {
            width: width as f32,
            height: height as f32,
            static_background: RGBACanvas::new(width, height),
            shapes: Vec::new(),
            walls: Vec::new(),
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

        for i in 0..self.walls.len() {
            self.walls[i].draw_simple(&mut background);
        }
        self.static_background = background;
        self.is_updated = true;
    }

    fn add_shapes(&mut self) {
        // maybe for dynamic objects

    }
    

    pub fn add_shapes_as_walls(&mut self, shapes: &Vec<Shape>) {
        // redo adding shapes:
        // for static objects it is better to have them as flat array of Vector2D's
        // this will then be simpler for collisions

        for j in 0..shapes.len() {
            for i in 0..shapes[j].elements.len() {
                self.walls.push(shapes[j].elements[i].new_shifted(shapes[j].anchor));
            }
        }
        
        self.create_static_background();
    }

    pub fn get_local_walls(&self, location: Coord, range: f32) -> Vec<Vector2D> {
        // this function is supposed to return only walls that are local to a point,
        // that is, theoretically visible to agent
        // get all walls that have bases or tips in the provided range
        // so that invisible walls are not checked for intersection with sweeping ray


        let local_walls: Vec<Vector2D> = Vec::new();


        return local_walls;
    }
    
}