// here, the object describing the world

use crate::{
    common_structs::{
        Coord,
    },
    rgba_canvas::RGBACanvas,
    shape::Shape,
    vector_2d::Vector2D,
};


pub struct World {
    pub shapes: Vec<Shape>,
    pub walls: Vec<Vector2D>,
    pub is_updated: bool,
}

impl World {
    pub fn new() -> World {
        let mut new_world: World = World {
            shapes: Vec::new(),
            walls: Vec::new(),
            is_updated: true,
        };

        new_world.add_shapes();

        return new_world;
    }

/////////////////////////////////////////////////////////

    pub fn render_top_view(&self, agent_shape: &Shape, center: Coord, scale: f32, canvas_width: i32, canvas_height: i32) -> RGBACanvas{
        // create top view for the world that is scaled and shifted
        //
        // centering:
        // center coord should be mapped to the center of canvas
        // somewhat roundabout way: having center coordinate, the dimentions of canvas and scale
        // find coordinate of old origin relative to new origin

        let mut rendered_view: RGBACanvas = RGBACanvas::new_black(canvas_width, canvas_height);
        let new_origin: Coord = Coord::new(
            -(center.x() - ((canvas_width / 2) as f32) / scale),
            -(center.y() - ((canvas_height / 2) as f32) / scale),
        );

        for i in 0..self.walls.len() {
            self.walls[i].draw_simple_s(&mut rendered_view, new_origin, scale);
            // self.walls[i].draw_smooth(&mut rendered_view);
        }

        for i in 0..agent_shape.elements.len() {
            agent_shape.elements[i].new_shifted(agent_shape.anchor).draw_simple_s(&mut rendered_view, new_origin, scale);   
        }

        return rendered_view;
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
    }

    pub fn get_local_walls(&self, location: Coord, range: f32) -> Vec<Vector2D> {
        // this function is supposed to return only walls that are local to a point,
        // that is, theoretically visible to agent
        // get all walls that have bases or tips in the provided range
        // so that invisible walls are not checked for intersection with sweeping ray
        //
        // 
        //


        let local_walls: Vec<Vector2D> = Vec::new();


        return local_walls;
    }
    
}