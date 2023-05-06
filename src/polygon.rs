// this is a versatile structure that can be used to describe different closed shapes

use crate::{common_structs::{Coord, Angle, RGBAColor}, line::Line, rgba_canvas::RGBACanvas};

pub enum PType {
  Regular{n: usize, r: f64, pivot: Coord, color: RGBAColor},
  // Box{length: f64, width: f64, pivot: Coord, color: RGBAColor},
  // RegElliptic{n: usize, length: f64, width: f64, pivot: Coord, color: RGBAColor},
  // Convex{pivot: Coord, vertices: Vec<Coord>, color: RGBAColor},
  // Random{pivot: Coord, vertices: Vec<Coord>, color: RGBAColor},
}

// Polygon is a collection of points connected with line segments in closed loop
// These line segments are not intersecting
#[derive(Clone)]
pub struct Polygon {
  name: String,
  pivot: Coord, // global coordinates
  vertices: Vec<Coord>, // relative to pivot
  pub sides: Vec<Line>, // lines defined in global coordinates 
  pub angle: Angle, // relative to global x axis
}

impl Polygon {
    pub fn new(kind: PType) -> Option<Polygon> {
      match kind {
        PType::Regular{n, r, pivot, color} => {
          if n < 3 && r <= 0.0 {
            return None;
          } else {
            let delta_alpha: f64 = std::f64::consts::TAU / (n as f64);
            let mut vertices: Vec<Coord> = Vec::with_capacity(n);
            let mut sides: Vec<Line> = Vec::with_capacity(n);

            for i in 0..n {
              vertices.push(Coord::new(f64::cos(delta_alpha * (i as f64)) * r, f64::sin(delta_alpha * (i as f64)) * r));
            }

            for i in 0..n { 
              sides.push(
                Line::new(
                  Coord::new(vertices[i].x() + pivot.x(), vertices[i].y() + pivot.y()),
                  Coord::new(vertices[(i + 1) % n].x() + pivot.x(), vertices[(i + 1) % n].y() + pivot.y()), color)
              );
            }

            let new_polygon = Polygon {
              name: String::from(""),
              pivot,
              vertices,
              sides,
              angle: Angle::new(),
            };

            return Some(new_polygon);
          }
        }
        /* PType::Box(length, width, pivot, color) => {

        } */
      }
    }

    pub fn rotate(&mut self, alpha: Angle) {
      let mut rot_vertice: Coord;
      let mut rot_vertice_next: Coord;

      self.angle.turn(alpha.get_value());

      for i in 0..self.vertices.len() {
        rot_vertice = self.vertices[i].rotate(self.angle);
        rot_vertice_next = self.vertices[(i + 1) % self.vertices.len()].rotate(self.angle);

        self.sides[i].start.set_x(rot_vertice.x() + self.pivot.x());
        self.sides[i].start.set_y(rot_vertice.y() + self.pivot.y());

        self.sides[i].end.set_x(rot_vertice_next.x() + self.pivot.x());
        self.sides[i].end.set_y(rot_vertice_next.y() + self.pivot.y());
      }
    }

    pub fn draw(&self, canvas: &mut RGBACanvas) {
      for i in 0..self.sides.len() {
        self.sides[i].draw(canvas);
        // self.sides[i].draw_line_p(canvas);
      }
    }
}