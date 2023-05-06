use crate::common_structs::RGBAColor;

#[derive(Clone)]
pub struct RGBACanvas {
    pub width: i32,
    u_width: usize,
    pub height: i32,
    u_height: usize,
    pub data: Vec<u8>,
}

impl RGBACanvas {
    pub fn new(width: i32, height: i32) -> RGBACanvas {
        let u_width: usize = if width > 0 {width as usize} else {panic!("Canvas width should be positive non-zero integer")};
        let u_height: usize = if height > 0 {height as usize} else {panic!("Canvas height should be positive non-zero integer")};

        return RGBACanvas {
            width,
            u_width,
            height,
            u_height,
            data: vec![0; u_width * u_height * 4],
        };
    }

    pub fn new_black(width: i32, height: i32) -> RGBACanvas {
        let u_width: usize = if width > 0 {width as usize} else {panic!("Canvas width should be positive non-zero integer")};
        let u_height: usize = if height > 0 {height as usize} else {panic!("Canvas height should be positive non-zero integer")};

        let mut data: Vec<u8> = vec![0; u_width * u_height * 4];
        for i in 0..(u_width * u_height) {
            data[i * 4 + 3] = 255;
        }

        return RGBACanvas {
            width,
            u_width,
            height,
            u_height,
            data,
        };
    }

    pub fn new_color(width: i32, height: i32, color: RGBAColor) -> RGBACanvas {
        let u_width: usize = if width > 0 {width as usize} else {panic!("Canvas width should be positive non-zero integer")};
        let u_height: usize = if height > 0 {height as usize} else {panic!("Canvas height should be positive non-zero integer")};

        let mut data: Vec<u8> = vec![0; u_width * u_height * 4];
        for i in 0..(u_width * u_height) {
            data[i * 4 + 0] = color.r;
            data[i * 4 + 1] = color.g;
            data[i * 4 + 2] = color.b;
            data[i * 4 + 3] = 255;
        }

        return RGBACanvas {
            width,
            u_width,
            height,
            u_height,
            data,
        };
    }

    pub fn new_f(width: f64, height: f64) -> RGBACanvas {
        let u_width: usize = if width >= 1.0 {width as usize} else {panic!("Canvas width should be a number bigger than 1.0")};
        let u_height: usize = if height >= 1.0 {height as usize} else {panic!("Canvas height should be a number bigger than 1.0")};

        let mut data: Vec<u8> = vec![0; u_width * u_height * 4];
        for i in 0..(u_width * u_height) {
            data[i * 4 + 3] = 255;
        }

        return RGBACanvas {
            width: u_width as i32,
            u_width,
            height: u_height as i32,
            u_height,
            data,
        };
    }

    pub fn copy(&self) -> RGBACanvas {
        let mut copied_data: Vec<u8> = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            copied_data.push(self.data[i]);
        }

        return RGBACanvas {
            width: self.width,
            u_width: self.u_width,
            height: self.height,
            u_height: self.u_height,
            data: copied_data,
        };
    }

    pub fn get_color(&self, x: i32, y: i32) -> RGBAColor {
        let index: usize = 4 * self.u_width * y as usize + 4 * x as usize;

        return RGBAColor::new_rgba(
            self.data[index + 0],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        );
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: RGBAColor) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            
            let index: usize = 4 * self.u_width * y as usize + 4 * x as usize;
            let mixed_color: RGBAColor;

            if color.a == 255 {
                self.data[index + 0] = color.r;
                self.data[index + 1] = color.g;
                self.data[index + 2] = color.b;
                self.data[index + 3] = color.a;
            } else {
                mixed_color = RGBAColor::mix_colors(color, self.get_color(x, y));

                self.data[index + 0] = mixed_color.r;
                self.data[index + 1] = mixed_color.g;
                self.data[index + 2] = mixed_color.b;
                self.data[index + 3] = mixed_color.a;
            }
        }
        // else just ignore pixels outside of canvas borders
    }
}