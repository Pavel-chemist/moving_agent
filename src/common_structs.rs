#[derive(Copy, Clone)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBAColor {
    pub fn new() -> RGBAColor {
        return RGBAColor{r: 0, g: 0, b: 0, a: 0};
    }

    pub fn new_black() -> RGBAColor {
        return RGBAColor{r: 0, g: 0, b: 0, a: 255};
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> RGBAColor {
        return RGBAColor{r, g, b, a};
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> RGBAColor {
        return RGBAColor{r, g, b, a: 255};
    }

    pub fn mix_colors(foreground: RGBAColor, background: RGBAColor) -> RGBAColor {
        // background is assumed to have alpha = 255, and so does the result
        let mut new_color: RGBAColor = RGBAColor::new();

        new_color.r = (((foreground.r as i32) * (foreground.a as i32) + (background.r as i32) * (255 - foreground.a as i32)) / 255) as u8;

        new_color.g = (((foreground.g as i32) * (foreground.a as i32) + (background.g as i32) * (255 - foreground.a as i32)) / 255) as u8;

        new_color.b = (((foreground.b as i32) * (foreground.a as i32) + (background.b as i32) * (255 - foreground.a as i32)) / 255) as u8;

        new_color.a = if foreground.a as i32 + background.a as i32 > 255 {255} else {foreground.a + background.a};

        return new_color;
    }
}

#[derive(Clone)]
pub struct RGBACanvas {
    pub width: i32,
    u_width: usize,
    pub height: i32,
    u_height: usize,
    pub data: Vec<u8>,
    // switch: bool,
}

impl RGBACanvas {
    pub fn new(width: i32, height: i32) -> RGBACanvas {
        let u_width: usize = if width > 0 {width as usize} else {panic!("Canvas width should be positive non-zero integer")};
        let u_height: usize = if width > 0 {width as usize} else {panic!("Canvas height should be positive non-zero integer")};

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
        let u_height: usize = if width > 0 {width as usize} else {panic!("Canvas height should be positive non-zero integer")};

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
        let u_height: usize = if width > 0 {width as usize} else {panic!("Canvas height should be positive non-zero integer")};

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
        let u_height: usize = if width >= 1.0 {width as usize} else {panic!("Canvas height should be a number bigger than 1.0")};

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

    fn get_color(&self, x: i32, y: i32) -> RGBAColor {
        let index: usize = 4 * self.u_width * y as usize + 4 * x as usize;

        return RGBAColor::new_rgba(
            self.data[index + 0],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        );
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: RGBAColor) {
        // this offset is needed somehow for proper alignment
        // it is a clutch
        let y_offset: i32 = 128;
        // let y_offset: i32 = 308;

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            
            let index: usize = 4 * self.u_width * (y_offset + y) as usize + 4 * x as usize;
            let mixed_color: RGBAColor;

            if color.a == 255 {
                self.data[index + 0] = color.r;
                self.data[index + 1] = color.g;
                self.data[index + 2] = color.b;
                self.data[index + 3] = color.a;
            } else {
                mixed_color = RGBAColor::mix_colors(color, self.get_color(x, (y_offset + y)));

                self.data[index + 0] = mixed_color.r;
                self.data[index + 1] = mixed_color.g;
                self.data[index + 2] = mixed_color.b;
                self.data[index + 3] = mixed_color.a;
            }
        }
        // else just ignore pixels outside of canvas borders
    }
}

#[derive(Copy, Clone)]
pub struct Coord {
    x: f64,
    y: f64,
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

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn x(&self) -> f64 {
        return self.x;
    }

    pub fn y(&self) -> f64 {
        return self.y;
    }

    pub fn get_x_i(&self) -> i32 {
        return self.x as i32;
    }

    pub fn get_y_i(&self) -> i32 {
        return self.y as i32;
    }

    pub fn move_x(&mut self, delta_x: f64) {
        self.x += delta_x;
    }

    pub fn move_y(&mut self, delta_y: f64) {
        self.y += delta_y;
    }

    pub fn rotate(&self, alpha: Angle) -> Coord {
        return Coord::new(
            f64::cos(alpha.a) * self.x - f64::sin(alpha.a) * self.y,
            f64::sin(alpha.a) * self.x + f64::cos(alpha.a) * self.y,
        );
    }
}

#[derive(Clone, Copy)]
pub struct Angle {
    a: f64,
}

impl Angle {
    pub fn new() -> Angle {
        return Angle{a: 0.0};
    }

    pub fn new_f(a: f64) -> Angle {
        return Angle{ a };
    }

    pub fn turn(&mut self, value: f64) {
        let mut new_angle: f64 = self.a + value;

        while new_angle > std::f64::consts::TAU {
            new_angle -= std::f64::consts::TAU;
        }

        while new_angle < 0.0 {
            new_angle += std::f64::consts::TAU;
        }

        self.a = new_angle;
    }

    pub fn get_value(&self) -> f64 {
        return self.a;
    }
}
