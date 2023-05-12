// this is to be an improvement and replacement of struct Line

use crate::{common_structs::{Coord, RGBAColor, Angle}, line_seg::LineSeg, linear_texture::LinearTexture, rgba_canvas::RGBACanvas};

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
    let v: Coord = Coord::new(
      segment.end.x() - segment.start.x(),
      segment.end.y() - segment.start.y(),
    );

    let length: f32 = f32::sqrt(v.x() * v.x() + v.y() * v.y());
    let phi: Angle = Angle::new_rad(f32::atan2(v.y(), v.x()));

    return Vector2D {
      base: segment.start,
      tip: v,
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

  // pub

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

  pub fn draw_simple(&self, canvas: &mut RGBACanvas) {
    // simple parametric drawing on canvas

    let delta_x: f32 = self.tip.x() / self.length;
    let delta_y: f32 = self.tip.y() / self.length;
    let mut current_pos: Coord = self.base;

    // canvas.put_pixel(current_pos.get_x_i(), current_pos.get_y_i(), self.texture.main_color);
    for _t in 0..(self.length as usize + 0) {
      current_pos.move_x(delta_x);
      current_pos.move_y(delta_y);

      canvas.put_pixel(current_pos.get_x_i(), current_pos.get_y_i(), self.texture.main_color);
    }
  }
}

