use crate::common_structs::RGBAColor;

#[derive(Copy, Clone)]
pub enum TextType {
  Sin,
  Lin,
  Step,
}

#[derive(Copy, Clone)]
pub enum TransType {
  Step,
  Lin,
  Quad,
}

#[derive(Copy, Clone)]
pub struct LinearTexture {
  main_color: RGBAColor,
  edge_color: RGBAColor,
  edge_width: f32,
  edge_transition_type: TransType,
  periodic_color: RGBAColor,
  period_length: f32,
  period_start_phase: f32, // 0.0..1.0
  period_type: TextType,
  period_fraction: f32, // 0.0..1.0
}

impl LinearTexture {
  pub fn new(
    main_color: RGBAColor,
    edge_color: RGBAColor,
    edge_width: f32,
    edge_transition_type: TransType,
    periodic_color: RGBAColor,
    period_length: f32,
    period_start_phase: f32,
    period_type: TextType,
    period_fraction: f32,
  ) -> LinearTexture {
    return LinearTexture {
      main_color,
      edge_color,
      edge_width,
      edge_transition_type,
      periodic_color,
      period_length,
      period_start_phase,
      period_type,
      period_fraction,
    };
  }

  pub fn new_plain(color: RGBAColor) -> LinearTexture {
    return LinearTexture {
      main_color: color,
      edge_color: color,
      edge_width: 0.0,
      edge_transition_type: TransType::Step,
      periodic_color: color,
      period_length: 0.0,
      period_start_phase: 0.0,
      period_type: TextType::Step,
      period_fraction: 0.0,
    };
  }

  pub fn new_shifted_phase(&self, shift: f32) -> LinearTexture {
    let mut updated_texture = *self;

    updated_texture.period_start_phase = (updated_texture.period_start_phase + shift / updated_texture.period_length).fract();
    return updated_texture;
  }

  pub fn add_edges(&mut self, color: RGBAColor, width: f32, transition: TransType) {
    self.edge_width = if width > 0.0 {width} else {0.0};
    self.edge_color = color;
    self.edge_transition_type = transition;
  }

  pub fn add_periodic_texture(&mut self, color: RGBAColor, length: f32, start_phase: f32, texture_type: TextType) {
    self.periodic_color = color;
    self.period_length = length;
    self.period_start_phase = start_phase;
    self.period_type = texture_type;
  }

  pub fn get_color(&self, length: f32, position: f32) -> RGBAColor {
    // length of whole vector,
    // position along this vector
    let mut color: RGBAColor = self.main_color;
    let mut opaqueness: u8;
    let is_edge: bool;
    let edge_fraction: f32;

    if length > 0.0 {
      if self.edge_width > 0.0 {
        if position < self.edge_width {
          edge_fraction = 1.0 - position / self.edge_width;
          is_edge = true;
        } else if position > length - self.edge_width {
          edge_fraction = 1.0 - (length - position) / self.edge_width;
          is_edge = true;
        } else {
          edge_fraction = 0.0;
          is_edge = false;
        }
      } else {
        edge_fraction = 0.0;
        is_edge = false;
      }
  
      if self.period_length > 0.0 {
        let pos_fraction: f32 = ((position / self.period_length) + self.period_start_phase).fract();

        match self.period_type {
          TextType::Step => {
            if pos_fraction < self.period_fraction {
              color = self.periodic_color;
            }
          }
          TextType::Lin => {
            if pos_fraction < self.period_fraction {
              opaqueness = (255.0 * pos_fraction / self.period_fraction ) as u8;
              color = RGBAColor::mix_colors(
                self.periodic_color.change_transparency(opaqueness),
                color,
              );
            } else {
              opaqueness = (255.0 * (1.0 - pos_fraction) / (1.0 - self.period_fraction) ) as u8;
              color = RGBAColor::mix_colors(
                self.periodic_color.change_transparency(opaqueness),
                color,
              );
            }
          }
          TextType::Sin => {
            opaqueness = ((f32::sin(pos_fraction * std::f32::consts::TAU) + 1.0) * 127.5) as u8;
            color = RGBAColor::mix_colors(
              self.periodic_color.change_transparency(opaqueness),
              color,
            );
          }
        }
      }
  
      // edges are added last to be visible always
      if is_edge {
        match self.edge_transition_type {
          TransType::Step => {
            color = self.edge_color;
          }
          TransType::Lin => {
            opaqueness = (255.0 * edge_fraction) as u8;
            color = RGBAColor::mix_colors(self.edge_color.change_transparency(opaqueness), color);
          }
          TransType::Quad => {
            opaqueness = (255.0 * edge_fraction * edge_fraction) as u8;
            color = RGBAColor::mix_colors(self.edge_color.change_transparency(opaqueness), color);
          }
        }
      }
    } else {
      // no color modification if length is set to zero
    }

    return color;
  }
}