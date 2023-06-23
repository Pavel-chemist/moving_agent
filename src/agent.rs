// struct agent
// a collection of polygons, lines and dots(?)
// polygons, lines and dots are to be replaced by more generic shape
// has orientation
// moves are relative to its current orientation

use crate::{
  common_structs::{
    Coord,
    Angle,
    RGBAColor,
    Palette,
  },
  vector_2d::Vector2D,
  linear_texture::{
    LinearTexture,
  },
  shape::Shape,
};

pub enum Direction {
  Forward,
  Backward,
  Left,
  Right,
}

pub struct Agent {
  pub center: Coord,
  angle: Angle,
  pub shape: Shape,
  step_size: f32,
  f_o_v: Angle, // field of view
  m_v_d: f32, // max view distance
  visible_walls: Vec<Vector2D>,
  pub is_updated: bool,
}

impl Agent {
  pub fn new(init_coord: Coord, init_angle: Angle, f_o_v: Angle) -> Agent {
    let mut shape: Shape = Shape::new_box(
      String::from("Agent's shape"),
      0.4,
      0.3,
      LinearTexture::new_plain(RGBAColor::new_p(Palette::Green)),
    ).unwrap();
    let inner_shape: Shape = Shape::new_regular_polygon(
      String::from("Agent's inner shape"),
      0.20,
      3,
      LinearTexture::new_plain(RGBAColor::new_p(Palette::Red)),
    ).unwrap();
    shape.add_shape(inner_shape);
    shape.shift(init_coord);
    shape.rotate(init_angle);


    return Agent{
      center: init_coord,
      angle: init_angle,
      shape,
      step_size: 0.05,
      f_o_v,
      m_v_d: 10.0,
      visible_walls: Vec::new(),
      is_updated: true,
    };
  }

  /* pub fn draw(&self, canvas: &mut RGBACanvas) {
    self.shape.draw(canvas);
  } */
  
  pub fn agent_move(&mut self, direction: Direction) {
    let directed_step: Coord;

    match direction {
      Direction::Forward => { directed_step = Coord::new(self.step_size, 0.0).new_rotated(self.angle); }
      Direction::Backward => { directed_step = Coord::new(-self.step_size, 0.0).new_rotated(self.angle); }
      Direction::Left => { directed_step = Coord::new(0.0, -self.step_size).new_rotated(self.angle); }
      Direction::Right => { directed_step = Coord::new(0.0, self.step_size).new_rotated(self.angle); }
    }

    // let directed_step: Coord = Coord::new(self.step_size, 0.0).new_rotated(self.angle);

    self.center = self.center.new_offset(directed_step);

    self.shape.shift(directed_step);

    self.collide();
  }

  pub fn turn_sideways(&mut self, degrees: f32) {
    self.angle.turn_deg(degrees);
    self.shape.rotate(Angle::new_deg(degrees));
  }

  pub fn update_visible_walls(&mut self, walls: Vec<Vector2D>) {
    self.visible_walls = walls;
  }

  pub fn get_view(&self, size: i32) -> Vec<RGBAColor> {
    let angle_between_rays: Angle = Angle::new_deg(self.f_o_v.get_deg() / size as f32);
    let mut view_line: Vec<RGBAColor> = Vec::with_capacity(size.abs() as usize);

    let mut ray1: Vector2D;

    let mut intersections_list_v: Vec<Vector2D>;
    let mut shortest_distance: f32;
    let mut current_distance: f32;
    let mut current_dot_index: usize = 0;
    let mut column_color: RGBAColor;

    ray1 = Vector2D::new(
      self.shape.anchor,
      Coord::new(self.m_v_d, 0.0),
      LinearTexture::new_plain(RGBAColor::new()),
    );
    ray1.rotate(self.angle);
    ray1.rotate(Angle::new_rad(-self.f_o_v.get_rad()/2.0));

    for view_column in 0..size {

      intersections_list_v = Vec::new();

      
      for i in 0..self.visible_walls.len() {
        match ray1.new_rotated(Angle::new_rad(angle_between_rays.get_rad() * view_column as f32)).intersect(&self.visible_walls[i]) {
          Some(int_vec) => {
            intersections_list_v.push(int_vec);
          }
          None => {}
        }
      }

      shortest_distance = self.m_v_d * 2.0;
      column_color = RGBAColor::new_p(Palette::Black);
      if intersections_list_v.len() != 0 {

        for i in 0..intersections_list_v.len() {

          current_distance = intersections_list_v[i].length();

          if current_distance < shortest_distance {
            shortest_distance = current_distance;
            current_dot_index = i;
          }
        }

        column_color = 
          intersections_list_v[current_dot_index]
            .texture
            .get_color(
              0.0,
              0.0,
            )
            .new_scaled(
              get_scaling_factor(
                intersections_list_v[current_dot_index].length(),
                self.m_v_d,
              )
            );
      }

      view_line.push(column_color);
    }
    
    return view_line;
  }

  fn collide(&mut self) {
    // simplest -- the agent collider is a circle
    // check if distance to any Vector2D is less than a radius
    // if true
    //   move agent by difference
    // else
    // check distances to wall ends to eliminate weirdness around corners

    let mut is_collided_to_wall: bool = false;
    // let mut collisions_count: i32 = 0;

    for i in 0..self.visible_walls.len() {
      match self.visible_walls[i].new_orthogonal_from_point(self.center) {
        Some(vec_to_wall) => {
          if vec_to_wall.length() < self.shape.radius {
            // collisions_count += 1;

            let dt: f32 = 1.0 - vec_to_wall.length() / self.shape.radius;

            self.center = self.center.new_offset(Coord::new(
              -dt * vec_to_wall.tip.x(),
              -dt * vec_to_wall.tip.y(),
            ));

            self.shape.shift(Coord::new(
              -dt * vec_to_wall.tip.x(),
              -dt * vec_to_wall.tip.y(),
            ));

            is_collided_to_wall = true;
          }
        }
        None => {}
      }
    }

    if !is_collided_to_wall {
      // check if in the vicinity of corner
      for i in 0..self.visible_walls.len() {
        let vec_to_corner: Vector2D = Vector2D::from_coord(
          Coord::new(
            self.visible_walls[i].base.x() - self.center.x(),
            self.visible_walls[i].base.y() - self.center.y(),
          ),
          LinearTexture::new_plain(RGBAColor::new()),
        );

        if vec_to_corner.length() < self.shape.radius {
          // collisions_count += 1;
          let dt: f32 = 1.0 - vec_to_corner.length() / self.shape.radius;

            self.center = self.center.new_offset(Coord::new(
              -dt * vec_to_corner.tip.x(),
              -dt * vec_to_corner.tip.y(),
            ));

            self.shape.shift(Coord::new(
              -dt * vec_to_corner.tip.x(),
              -dt * vec_to_corner.tip.y(),
            ));
        }
      }
    }

    // println!("Collided to {} walls", collisions_count);
  }
}

fn get_scaling_factor(dist: f32, max: f32) -> f32 {
  let scaled_distance: f32 = dist / max;

  return (1.0 - scaled_distance) * (1.0 - scaled_distance);
}