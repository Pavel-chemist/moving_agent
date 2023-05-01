// struct sprite
// ??? is a raster image that can be placed in any orientation
// or a combination of primitives?

enum Translation {
  F,
  B,
  R,
  L,
}

enum Rotation {
  R,
  L,
}

struct Sprite {
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