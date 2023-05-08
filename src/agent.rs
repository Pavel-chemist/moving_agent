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
  }, rgba_canvas::RGBACanvas
};

pub struct Agent {
  center: Coord,
  angle: Angle,
  dots: Vec<Dot>,
  lines: Vec<Line>,
  polygons: Vec<Polygon>,
}

impl Agent {
  pub fn new(init_coord: Coord, init_angle: Angle) -> Agent {
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

    polygons.push(Polygon::new(PType::Sector{
      radius: 250.0,
      start_angle: Angle::new_deg(-45.0),
      end_angle: Angle::new_deg(45.0),
      pivot: Coord::new_i(0, 0),
      color: RGBAColor::new_rgb(255, 255, 255),
    }).unwrap());

    polygons[1].move_pivot(Coord::new(-50.0, 0.0));

    return Agent{
      center: init_coord,
      angle: init_angle,
      dots,
      lines,
      polygons,
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
}