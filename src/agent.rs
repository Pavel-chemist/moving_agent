// struct agent
// a collection of polygons, lines and dots(?)
// has orientation
// moves are relative to its current orientation

use crate::{
  common_structs::{
    Coord, Dot, Angle, RGBAColor, Marker
  },
  line::Line,
  polygon::{
    Polygon,
    PType,
  }, rgba_canvas::RGBACanvas, world::World
};

pub struct Agent {
  center: Coord,
  angle: Angle,
  dots: Vec<Dot>,
  lines: Vec<Line>,
  polygons: Vec<Polygon>,
  f_o_v: Angle, // field of view
  m_v_d: f64, // max view distance
  // view: Vec<RGBAColor>,
  pub is_updated: bool,
}

impl Agent {
  pub fn new(init_coord: Coord, init_angle: Angle, f_o_v: Angle, m_v_d: f64) -> Agent {
    let mut dots: Vec<Dot> = Vec::new();
    let mut lines: Vec<Line> = Vec::new();
    let mut polygons: Vec<Polygon> = Vec::new();

    dots.push(Dot::new(Coord::new_i(50, 20), RGBAColor::new_rgb(255, 0, 0), Marker::Disc(10)));
    dots.push(Dot::new(Coord::new_i(50, -20), RGBAColor::new_rgb(255, 0, 0), Marker::Disc(10)));

    lines.push(Line::new(Coord::new_i(-10, 0), Coord::new_i(70, 0), RGBAColor::new_rgb(0, 0, 255)));

    polygons.push(Polygon::new(PType::Rectangle{
      length: 140.0,
      width: 70.0,
      pivot: Coord::new_i(0, 0),
      color: RGBAColor::new_rgb(0, 255, 0),
    }).unwrap());

    // field of view
    polygons.push(Polygon::new(PType::Sector{
      radius: m_v_d,
      start_angle: Angle::new_deg(-(f_o_v.get_deg() / 2.0)),
      end_angle: Angle::new_deg((f_o_v.get_deg() / 2.0)),
      pivot: Coord::new_i(0, 0),
      color: RGBAColor::new_rgba(255, 255, 255, 127),
    }).unwrap());

    polygons[1].move_pivot(Coord::new(-50.0, 0.0));

    return Agent{
      center: init_coord,
      angle: init_angle,
      dots,
      lines,
      polygons,
      f_o_v,
      m_v_d,
      is_updated: true,
    };
  }

  pub fn draw(&self, canvas: &mut RGBACanvas) {
    for d in 0..self.dots.len() {
      self.dots[d].new_rot_offset(self.angle, self.center).draw(canvas);
    }

    let mut rot_offset_line: Line;
    for l in 0..self.lines.len() {
      rot_offset_line = Line::new(
        self.lines[l].start.new_rotated(self.angle).new_offset(self.center),
        self.lines[l].end.new_rotated(self.angle).new_offset(self.center),
        self.lines[l].color,
      );

      rot_offset_line.draw(canvas);
    }

    for p in 0..self.polygons.len() {
      self.polygons[p].new_rot_offset(self.angle, self.center).draw(canvas);
    }
  }
  
  pub fn move_forward(&mut self, step_size: f64) {
    let directed_step: Coord = Coord::new(step_size, 0.0).new_rotated(self.angle);

    self.center = self.center.new_offset(directed_step);
  }

  pub fn move_sideways(&mut self, step_size: f64) {
    let directed_step: Coord = Coord::new(0.0, step_size).new_rotated(self.angle);

    self.center = self.center.new_offset(directed_step);
  }

  pub fn turn_sideways(&mut self, degrees: f64) {
    self.angle.turn_deg(degrees);
  }

  pub fn get_view(&self, size: i32, world: &World) -> Vec<RGBAColor> {
    let angle_between_rays: Angle = Angle::new_deg(self.f_o_v.get_deg() / size as f64);
    let mut view_line: Vec<RGBAColor> = Vec::with_capacity(size.abs() as usize);
    let mut ray: Line;
    let mut sweeping_ray: Line;
    let mut intersections_list: Vec<Dot>;
    let mut shortest_distance: f64;
    let mut current_distance: f64;
    let mut current_dot_index: usize = 0;
    let mut column_color: RGBAColor;

    for view_column in 0..size {
      intersections_list = Vec::new();

      ray = Line::new(
        Coord::new_i(0 + 50, 0),
        Coord::new(self.m_v_d + 50.0, 0.0),
        RGBAColor::new_rgb(255,0,255),
      ).new_rot_offset_line(
        Angle::new_rad(-self.f_o_v.get_rad() / 2.0 + angle_between_rays.get_rad() * (view_column as f64)),
        self.polygons[1].pivot,
      );
  
      sweeping_ray = Line::new(
        ray.start.new_rotated(self.angle).new_offset(self.center),
        ray.end.new_rotated(self.angle).new_offset(self.center),
        ray.color,
      );

      for j in 0..world.shapes.len() {
        for i in 0..world.shapes[j].sides.len() {
          match sweeping_ray.intersection(&world.shapes[j].sides[i]) {
            Some(point) => {intersections_list.push(point)}
            None => {}
          };
        }
      }

      shortest_distance = self.m_v_d * 2.0;
      column_color = RGBAColor::new_black();
      if intersections_list.len() != 0 {
        for i in 0..intersections_list.len() {
          current_distance = Line::new(sweeping_ray.start, intersections_list[i].coord, intersections_list[i].color).get_length();
  
          if current_distance < shortest_distance {
            shortest_distance = current_distance;
            current_dot_index = i;
          }
        }
  
        column_color = intersections_list[current_dot_index].color.new_scaled(get_scaling_factor(shortest_distance, self.m_v_d));
      }

      view_line.push(column_color);

      /* for i in view.height/2..view.height/2+1 {
        view.put_pixel(
          view_column,
          i,
          column_color,
        );
      } */
    }
    
    return view_line;
  }
}

fn get_scaling_factor(dist: f64, max: f64) -> f64 {
  let scaled_distance: f64 = dist / max;

  return (1.0 - scaled_distance) * (1.0 - scaled_distance);
}