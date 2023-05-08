// struct sprite
// ??? is a raster image that can be placed in any orientation
// or a combination of primitives?

enum Translation {
  F(f64),
  B(f64),
  R(f64),
  L(f64),
}

enum Rotation {
  R(f64),
  L(f64),
}

struct Agent {
  center: Coord,
  height: i32,
  width: i32,
  angle: f64,
  // image: RGBACanvas,

}

impl Sprite {
  pub fn translation(&mut self, mov: Translation) {

  }
}