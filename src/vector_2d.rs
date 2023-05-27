// this is to be an improvement and replacement of struct Line

use crate::{
  common_structs::{
    Coord,
    RGBAColor,
    Angle,
    Dot,
    Marker,
    Palette,
  },
  linear_texture::LinearTexture,
  rgba_canvas::RGBACanvas,
};

#[derive(Copy, Clone)]
pub struct Vector2D {
  pub base: Coord,  // starting point
  pub tip: Coord, // vector value relative to starting point
  pub texture: LinearTexture,
  length: f32,
  phi: Angle,
}

impl Vector2D {
  pub fn new(base: Coord, tip: Coord, texture: LinearTexture) -> Vector2D {
    let length: f32 = f32::sqrt(tip.x() * tip.x() + tip.y() * tip.y());
    let phi: Angle = Angle::new_rad(f32::atan2(tip.y(), tip.x()));

    return Vector2D {base, tip, texture, length, phi};
  }

  pub fn from_scalar(length: f32, texture: LinearTexture) -> Vector2D {
    return Vector2D {
      base: Coord::new(0.0, 0.0),
      tip: Coord::new(length, 0.0),
      texture,
      length,
      phi: Angle::new(),
    };
  }

  pub fn from_coord(tip: Coord, texture: LinearTexture) -> Vector2D {
    let length: f32 = f32::sqrt(tip.x() * tip.x() + tip.y() * tip.y());
    let phi: Angle = Angle::new_rad(f32::atan2(tip.y(), tip.x()));

    return Vector2D {
      base: Coord::new_i(0, 0),
      tip,
      texture,
      length,
      phi,
    };
  }

  pub fn new_scaled(&self, scale_factor: f32) -> Vector2D {
    let v: Coord = Coord::new(
      self.tip.x() * scale_factor,
      self.tip.y() * scale_factor,
    );

    let length: f32 = self.length * scale_factor;

    return Vector2D {
      base: self.base,
      tip: v,
      texture: self.texture,
      length,
      phi: self.phi,
    };
  }

  pub fn scale(&mut self, scale_factor: f32) {
    self.base = Coord::new(self.tip.x() * scale_factor, self.tip.y() * scale_factor);
    self.length = self.length * scale_factor;
  }

  pub fn new_rotated(&self, alpha: Angle) -> Vector2D {
    let phi: Angle = Angle::new_rad(self.phi.get_rad() + alpha.get_rad());
    let v: Coord = Coord::new(
      self.length() * f32::cos(phi.get_rad()),
      self.length() * f32::sin(phi.get_rad()),
    );

    return Vector2D {
      base: self.base,
      tip: v,
      texture: self.texture,
      length: self.length,
      phi,
    };
  }

  pub fn rotate(&mut self, alpha: Angle) {
    self.phi.turn(alpha);
    self.tip = Coord::new(
      self.length() * f32::cos(self.phi.get_rad()),
      self.length() * f32::sin(self.phi.get_rad()),
    );
  }

  pub fn new_shifted(&self, shift: Coord) -> Vector2D {
    return Vector2D {
      base: self.base.new_offset(shift),
      tip: self.tip,
      texture: self.texture,
      length: self.length,
      phi: self.phi,
    };
  }

  pub fn shift(&mut self, shift: Coord) {
    self.base = self.base.new_offset(shift);
  }

  pub fn shange_base(&mut self, new_base: Coord) {
    self.base = new_base;
  }

  pub fn add(&self, addend: Vector2D) -> Vector2D {
    // vector addition, creates a new vector
    // base and texture are inherited from first vector

    let tip: Coord = addend.tip.new_offset(self.tip);
    let length: f32 = f32::sqrt(tip.x() * tip.x() + tip.y() * tip.y());
    let phi: Angle = Angle::new_rad(f32::atan2(tip.y(), tip.x()));

    return Vector2D {
      base: self.base,
      tip,
      texture: self.texture,
      length,
      phi,
    };
  }

  pub fn sub(&self, addend: Vector2D) -> Vector2D {
    // vector subtraction, creates a new vector
    // base and texture are inherited from first vector

    let tip: Coord = addend.tip.new_offset(Coord::new(-self.tip.x(), -self.tip.y()));
    let length: f32 = f32::sqrt(tip.x() * tip.x() + tip.y() * tip.y());
    let phi: Angle = Angle::new_rad(f32::atan2(tip.y(), tip.x()));

    return Vector2D {
      base: self.base,
      tip,
      texture: self.texture,
      length,
      phi,
    };
  }

  pub fn reverse(&self) -> Vector2D {
    let reversed_tip: Coord = Coord::new(-self.tip.x(), -self.tip.y());

    return Vector2D {
      base: self.base,
      tip: reversed_tip,
      texture: self.texture,
      length: self.length,
      phi: self.phi.new_turned_rad(std::f32::consts::PI),
    };
  }

  pub fn get_unit_vector(&self) -> Vector2D {
    // returns vector of unit size, with the same base
    let tip: Coord = Coord::new(f32::cos(self.phi.get_rad()), f32::sin(self.phi.get_rad()));

    return Vector2D {
      base: self.base,
      tip,
      texture: self.texture,
      length: 1.0,
      phi: self.phi,
    };
  }

  pub fn get_normal(&self) -> Vector2D {
    // returns orthogonal vector of same size, with the same base
    let phi: Angle = self.phi.new_turned_rad(std::f32::consts::FRAC_PI_2);
    let tip: Coord = Coord::new(
      -self.tip.y(),
      self.tip.x(),
    );

    return Vector2D {
      base: self.base,
      tip,
      texture: self.texture,
      length: self.length,
      phi,
    };
  }

  pub fn change_texture(&mut self, new_texture: LinearTexture) {
    self.texture = new_texture;
  }

  pub fn intersect(&self, other: &Vector2D) -> Option<Vector2D> {
    let t: f32; // parameter along self 0.0..1.0
    let u: f32; // parameter along other 0.0..1.0
    let det: f32 = self.tip.x() * other.tip.y() - self.tip.y() * other.tip.x();
    let d_b_x: f32; // difference between x coordinates of bases
    let d_b_y: f32; // difference between y coordinates of bases

    if det != 0.0 {
      d_b_x = self.base.x() - other.base.x();
      d_b_y = self.base.y() - other.base.y();

      t = (d_b_y * other.tip.x() - d_b_x * other.tip.y()) / det;
      u = (d_b_y * self.tip.x() - d_b_x * self.tip.y()) / det;

      if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
        // lines intersect
        return Some(
          Vector2D::from_scalar(
            t * self.length,
            LinearTexture::new_plain(
              other.texture.get_color(other.length, u * other.length),
            ),
          ),
        );        
      }
    }

    return None;
  }

  pub fn new_orthogonal_from_point(&self, point: Coord) -> Option<Vector2D> {
    // find a point at which self intersects with normal going through a given coordinate
    // get normal throught the point
    // calculate intersection
    // 

    let mut nvtp: Vector2D = self.get_normal(); // normal vector through point
    nvtp.base = point;

    let t: f32; // parameter along self 0.0..1.0
    let u: f32; // parameter along other 0.0..1.0 (other is normal going from given point)
    let det: f32 = self.tip.x() * nvtp.tip.y() - self.tip.y() * nvtp.tip.x();

    let d_b_x: f32 = self.base.x() - nvtp.base.x();
    let d_b_y: f32 = self.base.y() - nvtp.base.y();

    t = (d_b_y * nvtp.tip.x() - d_b_x * nvtp.tip.y()) / det;
    u = (d_b_y * self.tip.x() - d_b_x * self.tip.y()) / det;


    if t >= 0.0 && t <= 1.0 {
      // normal is intersecing somewhere along self
      return Some(Vector2D::new(
        point,
        Coord::new(
          u * nvtp.tip.x(),
          u * nvtp.tip.y(),
        ),
        LinearTexture::new_plain(
          self.texture.get_color(self.length, t * self.length),
        ),
      ));
    } else {
      return None;
    }
  }

  pub fn length(&self) -> f32 {
    return self.length;
  }

  pub fn phi(&self) -> Angle {
    return self.phi;
  }

  pub fn is_from_origin(&self) -> bool {
    return self.base.x() == 0.0 && self.base.y() == 0.0;
  }

  pub fn replace_texture(&mut self, new_texture: LinearTexture) {
    self.texture = new_texture;
  }

  pub fn print_values(&self) {
    println!("Base: x= {:.1}, y= {:.1}; length: {:.1}; phi: {:.1} degrees.",
      self.base.x(), self.base.y(), self.length, self.phi.get_deg());
  }

  pub fn draw_simple(&self, canvas: &mut RGBACanvas) {
    // simple parametric drawing on canvas

    let delta_x: f32 = self.tip.x() / self.length;
    let delta_y: f32 = self.tip.y() / self.length;
    let mut current_pos: Coord = self.base;

    // canvas.put_pixel(current_pos.get_x_i(), current_pos.get_y_i(), self.texture.main_color);
    for t in 0..(self.length as usize + 0) {
      current_pos.move_x(delta_x);
      current_pos.move_y(delta_y);

      canvas.put_pixel(current_pos.get_x_i(), current_pos.get_y_i(), self.texture.get_color(self.length, t as f32));
    }
  }

  pub fn draw_smooth(&self, canvas: &mut RGBACanvas) {
    // draw smooth line on canvas
    // inefficient variant:
    // get axis aligned box
    // iterate through it, with little padding
    // for each point, find distance to line (the normal is needed)
    // if less than 1.0, get color, and scale it with 1.0 - distance,
    // and put on canvas

    if self.tip.x() >= 0.0 && self.tip.y() >= 0.0 {
      for j in (self.base.y() as i32 - 1)..((self.base.y() + self.tip.y()) as i32 + 2) {
        for i in (self.base.x() as i32 - 1)..((self.base.x() + self.tip.x()) as i32 + 2) {
          match self.new_orthogonal_from_point(Coord::new_i(i, j)) {
            Some(orth) => {
              if orth.length < 1.0 {
                canvas.put_pixel(i, j, orth.texture.get_color(0.0, 0.0).new_scaled(1.0 - orth.length));
              }
            }
            None => {}
          };
        }
      }
    } else if self.tip.x() < 0.0 && self.tip.y() >= 0.0 {
      for j in (self.base.y() as i32 - 1)..((self.base.y() + self.tip.y()) as i32 + 2) {
        for i in ((self.base.x() + self.tip.x()) as i32 - 1)..(self.base.x() as i32 + 2) {
          match self.new_orthogonal_from_point(Coord::new_i(i, j)) {
            Some(orth) => {
              if orth.length < 1.0 {
                canvas.put_pixel(i, j, orth.texture.get_color(0.0, 0.0).new_scaled(1.0 - orth.length));
              }
            }
            None => {}
          };
        }
      }
    } else if self.tip.x() >= 0.0 && self.tip.y() < 0.0 {
      for j in ((self.base.y() + self.tip.y()) as i32 - 1)..(self.base.y() as i32 + 2) {
        for i in (self.base.x() as i32 - 1)..((self.base.x() + self.tip.x()) as i32 + 2) {
          match self.new_orthogonal_from_point(Coord::new_i(i, j)) {
            Some(orth) => {
              if orth.length < 1.0 {
                canvas.put_pixel(i, j, orth.texture.get_color(0.0, 0.0).new_scaled(1.0 - orth.length));
              }
            }
            None => {}
          };
        }
      }
    } else /* if self.tip.x() < 0.0 && self.tip.y() < 0.0 */ {
      for j in ((self.base.y() + self.tip.y()) as i32 - 1)..(self.base.y() as i32 + 2) {
        for i in ((self.base.x() + self.tip.x()) as i32 - 1)..(self.base.x() as i32 + 2) {
          match self.new_orthogonal_from_point(Coord::new_i(i, j)) {
            Some(orth) => {
              if orth.length < 1.0 {
                canvas.put_pixel(i, j, orth.texture.get_color(0.0, 0.0).new_scaled(1.0 - orth.length));
              }
            }
            None => {}
          };
        }
      }
    }

  }

  /* pub fn get_quad(&self) -> Quad {
    if self.tip.x() >= 0.0 && self.tip.y() >= 0.0 {
      return Quad::TR;
    } else if self.tip.x() < 0.0 && self.tip.y() >= 0.0 {
      return Quad::TL;
    } else if self.tip.x() >= 0.0 && self.tip.y() < 0.0 {
      return Quad::BR;
    } else {
      return Quad::BL;
    }
  } */
}

/* pub enum Quad {
  TR, //top right
  TL, //top left
  BR, // bottom right
  BL, // bottom left
} */

