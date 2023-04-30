#[derive(Copy, Clone)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct RGBCanvas {
    pub width: f64,
    pub height: f64,
    pub data: Vec<u8>,
}

impl RGBCanvas {
    pub fn new(width: f64, height: f64) -> RGBCanvas {
        return RGBCanvas {
            width,
            height,
            data: vec![0; (width * height * 3.0) as usize],
        };
    }

    pub fn copy(&self) -> RGBCanvas {
        let mut copied_data: Vec<u8> = vec![0; self.data.len()];

        for i in 0..self.data.len() {
            copied_data[i] = self.data[i];
        }

        return RGBCanvas {
            width: self.width,
            height: self.height,
            data: copied_data,
        };
    }
}

pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        return Coord {x, y};
    }

    pub fn new_i(x: i32, y: i32) -> Coord {
        return Coord {
            x: x as f64,
            y: y as f64,
        };
    }

    pub fn set_i(&mut self, x: i32, y: i32) {
        self.x = x as f64;
        self.y = y as f64;
    }
}