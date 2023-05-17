// struct agent
// a collection of polygons, lines and dots(?)
// polygons, lines and dots are to be replaced by more generic shape
// has orientation
// moves are relative to its current orientation

use crate::{
  common_structs::{
    Coord, Dot, Angle, RGBAColor, Marker, Palette
  },
  line_seg::LineSeg,
  polygon::{
    Polygon,
    PType,
  }, rgba_canvas::RGBACanvas, world::World, vector_2d::Vector2D, linear_texture::{TextType, TransType, LinearTexture}, shape::Shape
};

pub struct Agent {
  center: Coord,
  angle: Angle,
  shape: Shape,
  f_o_v: Angle, // field of view
  m_v_d: f32, // max view distance
  pub is_updated: bool,
}

impl Agent {
  pub fn new(init_coord: Coord, init_angle: Angle, f_o_v: Angle, m_v_d: f32) -> Agent {
    let mut shape: Shape = Shape::new_box(
      String::from("Agent's shape"),
      50.0,
      40.0,
      LinearTexture::new_plain(RGBAColor::new_p(Palette::Green)),
    ).unwrap();
    let inner_shape: Shape = Shape::new_regular_polygon(
      String::from("Agent's inner shape"),
      20.0,
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
      f_o_v,
      m_v_d,
      is_updated: true,
    };
  }

  pub fn draw(&self, canvas: &mut RGBACanvas) {
    self.shape.draw(canvas);
  }
  
  pub fn move_forward(&mut self, step_size: f32) {
    let directed_step: Coord = Coord::new(step_size, 0.0).new_rotated(self.angle);

    self.center = self.center.new_offset(directed_step);
    self.shape.shift(directed_step);
  }

  pub fn move_sideways(&mut self, step_size: f32) {
    let directed_step: Coord = Coord::new(0.0, step_size).new_rotated(self.angle);

    self.center = self.center.new_offset(directed_step);
    self.shape.shift(directed_step);
  }

  pub fn turn_sideways(&mut self, degrees: f32) {
    self.angle.turn_deg(degrees);
    self.shape.rotate(Angle::new_deg(degrees));
  }

  pub fn get_view(&self, size: i32, world: &World) -> Vec<RGBAColor> {
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
      LinearTexture::new_plain(RGBAColor::new_rgba(0, 0, 0, 0)),
    );
    ray1.rotate(self.angle);
    ray1.rotate(Angle::new_rad(-self.f_o_v.get_rad()/2.0));

    for view_column in 0..size {

      intersections_list_v = Vec::new();

      for j in 0..world.shapes.len() {
        for i in 0..world.shapes[j].elements.len() {
          match ray1.new_rotated(Angle::new_rad(angle_between_rays.get_rad() * view_column as f32)).intersect(&world.shapes[j].elements[i].new_shifted(world.shapes[j].anchor)) {
            Some(int_vec) => {
              intersections_list_v.push(int_vec);
            }
            None => {}
          }
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

        column_color = intersections_list_v[current_dot_index].texture.get_color(0.0, 0.0).new_scaled(get_scaling_factor(intersections_list_v[current_dot_index].length(), self.m_v_d));
      }

      view_line.push(column_color);
    }
    
    return view_line;
  }
}

fn get_scaling_factor(dist: f32, max: f32) -> f32 {
  let scaled_distance: f32 = dist / max;

  return (1.0 - scaled_distance) * (1.0 - scaled_distance);
}