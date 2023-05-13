// this is to be an improvement and replacement of struct Line

use crate::{common_structs::{Coord, RGBAColor, Angle, Dot, Marker}, line_seg::LineSeg, linear_texture::LinearTexture, rgba_canvas::RGBACanvas};

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

  pub fn from_line_seg(segment: &LineSeg) -> Vector2D {
    let tip: Coord = Coord::new(
      segment.end.x() - segment.start.x(),
      segment.end.y() - segment.start.y(),
    );

    let length: f32 = f32::sqrt(tip.x() * tip.x() + tip.y() * tip.y());
    let phi: Angle = Angle::new_rad(f32::atan2(tip.y(), tip.x()));

    return Vector2D {
      base: segment.start,
      tip,
      texture: LinearTexture::new_plain(segment.color),
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
    // returns orthogonal vector of unit size, with the same base
    let phi: Angle = self.phi.new_turned_rad(std::f32::consts::FRAC_PI_2);
    let tip: Coord = Coord::new(f32::cos(phi.get_rad()), f32::sin(phi.get_rad()));

    return Vector2D {
      base: self.base,
      tip,
      texture: self.texture,
      length: 1.0,
      phi,
    };
  }

  pub fn change_texture(&mut self, new_texture: LinearTexture) {
    self.texture = new_texture;
  }

  pub fn intersect(&self, other: &Vector2D, is_print: bool) -> Option<Vector2D> {
    // returns a Vector2D that has same base as self, and tip at intersection point
    // while texture is plain color of the point of the other

    let mut is_intersecting: bool = false;
    let angle_difference: f32;
    let a_s: f32;
    let b_s: f32;
    let a_o: f32;
    let b_o: f32;
    let mut x_i: f32 = 0.0;
    let mut y_i: f32 = 0.0;

    let color_at_intersect: RGBAColor;

    let mut vector_to_intersect: Vector2D = self.clone();

    let vec_bases: Vector2D = Vector2D::new(
      self.base,
      Coord::new(
        other.base.x() - self.base.x(),
        other.base.y() - self.base.y(),
      ),
      self.texture,
    );

    if self.length + other.length > vec_bases.length {

      angle_difference = (other.phi.get_deg() - self.phi.get_deg()); //.round().abs();

      // if angle_difference > 90.0 && angle_difference < 270.0 && angle_difference != 180.0 {
      if angle_difference != 0.0 && angle_difference != 180.0 {

        if self.tip.x().abs() <= 0.0001 {
          // self is vertical

          if other.tip.x().abs() <= 0.0001 {
            // both  are vertical
            // no intersection
          } else {
            // only self is vertical

            a_o = other.tip.y() / other.tip.x();
            b_o = other.base.y() - a_o * other.base.x();

            x_i = self.base.x();
            if a_o.abs() > 0.0 {
              y_i = a_o * x_i + b_o;
            } else {
              y_i = b_o;
            }

            is_intersecting = true;
          }
        } else {

          if other.tip.x().abs() <= 0.0001 {
            // only other is vertical
           
            a_s = self.tip.y() / self.tip.x();
            b_s = self.base.y() - a_s * self.base.x();

            x_i = other.base.x();
            y_i = a_s * x_i + b_s;

            is_intersecting = true;
          } else {
            // both NOT vertical

            a_s = self.tip.y() / self.tip.x();
            b_s = self.base.y() - a_s * self.base.x();

            a_o = other.tip.y() / other.tip.x();
            b_o = other.base.y() - a_o * other.base.x();

            x_i = (b_o - b_s) / (a_s - a_o);   
            y_i = a_s * x_i + b_s;

            /* if is_print && a_o.abs() < 0.001 {
              println!("none vertical, intersect at x: {:.1}, y: {:.1}", x_i, y_i);
              println!("Other a_o: {}", a_o);
              println!("+----------------------+");
            } */

            is_intersecting = true;
          }
        }

      }
    }

    if is_intersecting {
      let colinear_other: Vector2D =  Vector2D::new(
        other.base,
        Coord::new(
          x_i - other.base.x(),
          y_i - other.base.y(),
        ),
        other.texture,
      );

      if other.length < colinear_other.length ||
         (other.phi.get_deg() - colinear_other.phi.get_deg()).trunc().abs() > 0.1 {
        // no intersection
        is_intersecting = false;
      } else {
        // color_at_intersect other.texture.get_color(other.length, distance_along_other);
        vector_to_intersect = Vector2D::new(
          self.base,
          Coord::new(
            x_i - self.base.x(),
            y_i - self.base.y(),
          ),
          self.texture,
        );

        if self.length < vector_to_intersect.length || 
           (self.phi.get_deg() - vector_to_intersect.phi.get_deg()).trunc().abs() > 0.1 {
          // no intersection

          is_intersecting = false;
        } else {
          // definitely intersection
          color_at_intersect = other.texture.get_color(other.length, colinear_other.length);
          vector_to_intersect.texture = LinearTexture::new_plain(color_at_intersect);

        }
      }
    }

    if is_intersecting {

      return Some(vector_to_intersect);
    } else {
      return None;
    }
  }

  pub fn get_distance(&self, point: Coord) -> Option<f32> {
    // find a point at which self intersects with normal going through a given coordinate
    let normal_is_interscting: bool = false;

    if normal_is_interscting {
      return Some(0.0);
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
    // let delta_r: f32 = 
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
  }
}

