pub fn place_square(image_data: &mut Vec<u8>, width: i32, x: i32, y: i32, size: i32, color: common_structs::RGBColor) {
  let mut index_r: usize;

  for j in (y - size / 2)..(y + size / 2) {
      for i in (x - size / 2)..(x + size / 2) {
          if i >= 0 && i < MAIN_IMAGE_WIDTH && j > 0 && j < MAIN_IMAGE_HEIGHT {
              index_r = (width * 3 * j + i * 3) as usize;

              image_data[index_r] = color.r;
              image_data[index_r + 1] = color.g;
              image_data[index_r + 2] = color.b;
          }
      }
  }
}

pub fn radial_gradient(
  image_data: &mut Vec<u8>,
  width: i32,
  height: i32,
  x: i32,
  y: i32,
  parameter: f32,
  color: common_structs::RGBColor,
) {
  let mut index_r: usize;
  let mut squared_distance: f32;
  let mut brightness: f32;
  let mut r_br: f32;
  let mut g_br: f32;
  let mut b_br: f32;

  for j in 0..height {
      for i in 0..width {
          index_r = (width * 3 * j + i * 3) as usize;
          squared_distance =
              (((x - i) * (x - i) + (y - j) * (y - j)) as f32) / (parameter * parameter);
          if squared_distance <= 1.0 {
              brightness = 1.0;
          } else {
              brightness = 1.0 / squared_distance;
          }

          r_br = brightness * (color.r as f32) + (image_data[index_r] as f32);
          g_br = brightness * (color.g as f32) + (image_data[index_r + 1] as f32);
          b_br = brightness * (color.b as f32) + (image_data[index_r + 2] as f32);

          if r_br > 255.0 {
              r_br = 255.0;
          }

          if b_br > 255.0 {
              b_br = 255.0;
          }

          if g_br > 255.0 {
              g_br = 255.0;
          }

          image_data[index_r] = r_br as u8;
          image_data[index_r + 1] = g_br as u8;
          image_data[index_r + 2] = b_br as u8;
      }
  }
}
