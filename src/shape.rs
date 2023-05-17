// this is generalization of polygon, based on vector_2d
// should supercede polygon

// the shape is collection of vectors, plus some parameters pertaining to shape as whole
// it does not need to be closed
// can

use crate::{common_structs::{Coord, Angle}, vector_2d::Vector2D, linear_texture::LinearTexture, rgba_canvas::RGBACanvas};

#[derive(Clone)]
pub struct Shape {
  name: String,
  pub elements: Vec<Vector2D>,
  c_o_m: Coord, // center of mass, relative to anchor -- a point in the center of shape for collision detections
  radius: f32, // distance from c_o_m to most distant point of the shape
  pub anchor: Coord, // point for rotations and translations
  // alpha: Ang
}

impl Shape {
  /* pub fn from_coord_array(name: String, vertices: Vec<Coord>, texture: LinearTexture) -> Shape {

  } */

  pub fn from_v2d(name: String, vec2d: Vector2D) -> Shape {
    let new_vec2d: Vector2D = Vector2D::new(
      Coord::new(-vec2d.tip.x() / 2.0, -vec2d.tip.y() / 2.0),
      Coord::new(vec2d.tip.x() / 2.0, vec2d.tip.y() / 2.0),
      vec2d.texture,
    );

    let elements: Vec<Vector2D> = vec![new_vec2d; 1];

    let radius: f32 = vec2d.length() / 2.0;
    let anchor: Coord = Coord::new(
      vec2d.base.x() + vec2d.tip.x() / 2.0,
      vec2d.base.y() + vec2d.tip.y() / 2.0,
    );
    let c_o_m: Coord = Coord::new(0.0, 0.0);

    return Shape {
      name,
      elements,
      c_o_m,
      radius,
      anchor,
    };
  }

  pub fn new_box(name: String, width: f32, height: f32, texture: LinearTexture) -> Option<Shape> {
    // creates a box with given dimensions and texture,
    // the texture wraps around clockwise
    // It is centered in origin and coordinate aligned

    if width > 0.0 && height > 0.0 {
      let mut elements: Vec<Vector2D> = Vec::with_capacity(4);
      let c_o_m: Coord = Coord::new(0.0, 0.0);
      let anchor: Coord = Coord::new(0.0, 0.0);
      let radius: f32 = f32::sqrt(width * width + height * height);
  
      // right side
      elements.push(Vector2D::new(
        Coord::new(width/2.0, -height/2.0),
        Coord::new(0.0, height),
        texture,
      ));
  
      // bottom side
      elements.push(Vector2D::new(
        Coord::new(width/2.0, height/2.0),
        Coord::new(-width, 0.0),
        texture.new_shifted_phase(height),
      ));
  
      // left side
      elements.push(Vector2D::new(
        Coord::new(-width/2.0, height/2.0),
        Coord::new(0.0, -height),
        texture.new_shifted_phase(width + height),
      ));
  
      // top side
      elements.push(Vector2D::new(
        Coord::new(-width/2.0, -height/2.0),
        Coord::new(width, 0.0),
        texture.new_shifted_phase(width + height * 2.0),
      ));
  
      return Some(Shape {
        name,
        elements,
        c_o_m,
        radius,
        anchor,
      });
    } else {
      println!("Width and Height of box shape should be bigger than zero!");

      return None;
    }
    
  }

  pub fn new_regular_polygon(name: String, radius: f32, num_sides: usize, texture: LinearTexture) -> Option<Shape> {
    if radius > 0.0 && num_sides > 2 {

      let mut elements: Vec<Vector2D> = Vec::with_capacity(num_sides);
      let c_o_m: Coord = Coord::new(0.0, 0.0);
      let anchor: Coord = Coord::new(0.0, 0.0);

      let delta_alpha: f32 = std::f32::consts::TAU / (num_sides as f32);
      let mut vertices: Vec<Coord> = Vec::with_capacity(num_sides);

      for i in 0..num_sides {
        vertices.push(Coord::new(
          f32::cos(delta_alpha * (i as f32)) * radius,
          f32::sin(delta_alpha * (i as f32)) * radius,
        ));
      }

      for i in 0..num_sides {
        elements.push(Vector2D::new(
          vertices[i],
          Coord::new(
            vertices[(i+1)%num_sides].x() - vertices[i].x(),
            vertices[(i+1)%num_sides].y() - vertices[i].y(),
          ),
          texture,
        ));
      }

      return Some(Shape {
        name,
        elements,
        c_o_m,
        radius,
        anchor,
      });
    } else {
      println!("Radius of Regular Polygon shape should be non-zero value, and it should have at least 3 sides!");

      return  None;
    }
  }

  pub fn add_shape(&mut self, added_shape: Shape) {
    for i in 0..added_shape.elements.len() {
      self.elements.push(added_shape.elements[i]);
    }
  }

  pub fn draw(&self, canvas: &mut RGBACanvas) {
    for i in 0..self.elements.len() {
      self.elements[i].new_shifted(self.anchor).draw_simple(canvas);
      // self.elements[i].new_shifted(self.anchor).draw_smooth(canvas);
    }
  }

  pub fn shift(&mut self, shift: Coord) {
    self.anchor = self.anchor.new_offset(shift);
  }

  pub fn rotate(&mut self, alpha: Angle) {
    //update all vectors comprizing the shape
    // rotate bases around anchor
    // and rotate tips around bases

    for i in 0..self.elements.len() {
      self.elements[i].base = self.elements[i].base.new_rotated(alpha);
      self.elements[i].tip = self.elements[i].tip.new_rotated(alpha);
    }
  }
}