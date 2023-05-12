use crate::common_structs::RGBAColor;

#[derive(Copy, Clone)]
pub enum TextType {
  Sin,
  Lin(f32), // number in range 0.0..1.0
  Step(f32), // number in range 0.0..1.0
}

#[derive(Copy, Clone)]
pub struct LinearTexture {
  pub main_color: RGBAColor,
  edge_color: RGBAColor,
  edge_width: f32,
  periodic_color: RGBAColor,
  period_length: f32,
  period_start_phase: f32,
  period_type: TextType,
}

impl LinearTexture {
  pub fn new(
    main_color: RGBAColor,
    edge_color: RGBAColor,
    edge_width: f32,
    periodic_color: RGBAColor,
    period_length: f32,
    period_start_phase: f32,
    period_type: TextType,
  ) -> LinearTexture {
    return LinearTexture {
      main_color,
      edge_color,
      edge_width,
      periodic_color,
      period_length,
      period_start_phase,
      period_type,
    };
  }

  pub fn new_plain(color: RGBAColor) -> LinearTexture {
    return LinearTexture {
      main_color: color,
      edge_color: color,
      edge_width: 0.0,
      periodic_color: color,
      period_length: 0.0,
      period_start_phase: 0.0,
      period_type: TextType::Step(0.0),
    };
  }
}