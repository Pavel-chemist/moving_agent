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

        new_color.a = 255;

        return new_color;;
    }
}

pub struct RGBACanvas {
    pub width: i32,
    pub height: i32,
    pub data: Vec<RGBAColor>,
}

impl RGBACanvas {
    pub fn new(width: i32, height: i32) -> RGBACanvas {
        let u_width: usize = if width > 0 {width as usize} else {panic!("Canvas width should be positive non-zero integer")};
        let u_height: usize = if width > 0 {width as usize} else {panic!("Canvas height should be positive non-zero integer")};

        return RGBACanvas {
            width,
            height,
            data: vec![RGBAColor::new_black(); u_width * u_height],
        };
    }

    pub fn new_f(width: f64, height: f64) -> RGBACanvas {
        let u_width: usize = if width >= 1.0 {width as usize} else {panic!("Canvas width should be a number bigger than 1.0")};
        let u_height: usize = if width >= 1.0 {width as usize} else {panic!("Canvas height should be a number bigger than 1.0")};

        return RGBACanvas {
            width: width as i32,
            height: height as i32,
            data: vec![RGBAColor::new_black(); u_width * u_height],
        };
    }

    pub fn copy(&self) -> RGBACanvas {
        let mut copied_data: Vec<RGBAColor> = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            copied_data.push(self.data[i]);
        }

        return RGBACanvas {
            width: self.width,
            height: self.height,
            data: copied_data,
        };
    }

    pub fn get_data_as_bytes(&self) -> Vec<u8> {
        let array_size: usize = self.data.len() * 4;

        let mut data_as_bytes: Vec<u8> = vec![0; array_size];

        for i in 0..self.data.len() {
            data_as_bytes[i * 4 + 0] = self.data[i].r;
            data_as_bytes[i * 4 + 1] = self.data[i].g;
            data_as_bytes[i * 4 + 2] = self.data[i].b;
            data_as_bytes[i * 4 + 3] = self.data[i].a;
        }

        return data_as_bytes;
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: RGBAColor) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            if color.a == 255 {
                self.data[(y * self.width + x) as usize] = color;
            } else {
                self.data[(y * self.width + x) as usize] = RGBAColor::mix_colors(color, self.data[(y * self.width + x) as usize]);
            }
        }
        // else just ignore pixels outside of canvas borders
    }

    pub fn draw_line_p(&mut self, line: Line) {
        // draw parametric line
        
        let x_s: i32 = line.start.get_x_i();
        let y_s: i32 = line.start.get_y_i();
        
        let mut delta_x: f64 = line.end.get_x() - line.start.get_x();
        let mut delta_y: f64 = line.end.get_y() - line.start.get_y();
        let mut t: f64;
    
        let step_x: f64;
        let step_y: f64;
    
        let mut x: i32;
        let mut y: i32;
    
        if delta_x < 0.0 { delta_x = - delta_x; }
        if delta_y < 0.0 { delta_y = - delta_y; }

        if delta_x >= delta_y {
            t = delta_x;
        } else {
            t = delta_y;
        }

        if t < 1.0 { t = 1.0; } 
    
        step_x = delta_x / t;
        step_y = delta_y / t;
    
        for t in 0..(t as usize) {
            x = x_s + (t as f64 * step_x) as i32;
            y = y_s + (t as f64 * step_y) as i32;

            self.put_pixel(x, y, line.color);
        }
    }

    /* pub fn draw_ellipse_p(&mut self, ellipse: &Ellipse) {
        // draw parametric ellipse
        let angular_steps: usize = ((ellipse.ver_axis  + ellipse.hor_axis) as usize);

        let mut x: i32;
        let mut y: i32;

        for j in 0..(ellipse.ver as usize) {
            x = 
        }
    } */

    pub fn draw_ellipse(&mut self, ellipse: &Ellipse) {
        let mut y: i32;
        let mut x: i32;

        if ellipse.angle == 0.0 {
            for x in 0..(ellipse.hor_axis as i32) {
                y = (ellipse.ver_axis * f64::sqrt(1.0 - ((x * x) as f64) / (ellipse.hor_axis * ellipse.hor_axis))) as i32;
    
                self.put_pixel(ellipse.center.x as i32 + x, ellipse.center.y as i32 + y, ellipse.color);
                self.put_pixel(ellipse.center.x as i32 - x, ellipse.center.y as i32 + y, ellipse.color);
                self.put_pixel(ellipse.center.x as i32 + x, ellipse.center.y as i32 - y, ellipse.color);
                self.put_pixel(ellipse.center.x as i32 - x, ellipse.center.y as i32 - y, ellipse.color);
            }
    
            for y in 0..(ellipse.ver_axis as i32) {
                x = (ellipse.hor_axis * f64::sqrt(1.0 - ((y * y) as f64) / (ellipse.ver_axis * ellipse.ver_axis))) as i32;
    
                self.put_pixel(ellipse.center.x as i32 + x, ellipse.center.y as i32 + y, ellipse.color);
                self.put_pixel(ellipse.center.x as i32 - x, ellipse.center.y as i32 + y, ellipse.color);
                self.put_pixel(ellipse.center.x as i32 + x, ellipse.center.y as i32 - y, ellipse.color);
                self.put_pixel(ellipse.center.x as i32 - x, ellipse.center.y as i32 - y, ellipse.color);
            }
        } else {
            let sin_alpha: f64 = f64::sin(ellipse.angle);
            let cos_alpha: f64 = f64::cos(ellipse.angle);

            let mut x: f64;
            let mut y: f64;
            let mut tr: Coord;
            let mut tl: Coord;
            let mut br: Coord;
            let mut bl: Coord;

            for i in 0..(ellipse.hor_axis as i32) {
                x = i as f64;
                y = ellipse.ver_axis * f64::sqrt(1.0 - x * x / (ellipse.hor_axis * ellipse.hor_axis));
                

                tr = Coord::new(x * cos_alpha - y * sin_alpha, x * sin_alpha + y * cos_alpha);
                tl = Coord::new(-x * cos_alpha - y * sin_alpha, -x * sin_alpha + y * cos_alpha);
                br = Coord::new(x * cos_alpha + y * sin_alpha, x * sin_alpha - y * cos_alpha);
                bl = Coord::new(-x * cos_alpha + y * sin_alpha, -x * sin_alpha - y * cos_alpha);

                self.put_pixel(ellipse.center.get_x_i() + tr.get_x_i(), ellipse.center.get_y_i() + tr.get_y_i(), ellipse.color);
                self.put_pixel(ellipse.center.get_x_i() + tl.get_x_i(), ellipse.center.get_y_i() + tl.get_y_i(), ellipse.color);
                self.put_pixel(ellipse.center.get_x_i() + br.get_x_i(), ellipse.center.get_y_i() + br.get_y_i(), ellipse.color);
                self.put_pixel(ellipse.center.get_x_i() + bl.get_x_i(), ellipse.center.get_y_i() + bl.get_y_i(), ellipse.color);
            }

            for j in 0..(ellipse.ver_axis as i32) {
                y = j as f64;
                x = ellipse.hor_axis * f64::sqrt(1.0 - (y * y) / (ellipse.ver_axis * ellipse.ver_axis));
    
                tr = Coord::new(x * cos_alpha - y * sin_alpha, x * sin_alpha + y * cos_alpha);
                tl = Coord::new(-x * cos_alpha - y * sin_alpha, -x * sin_alpha + y * cos_alpha);
                br = Coord::new(x * cos_alpha + y * sin_alpha, x * sin_alpha - y * cos_alpha);
                bl = Coord::new(-x * cos_alpha + y * sin_alpha, -x * sin_alpha - y * cos_alpha);

                self.put_pixel(ellipse.center.get_x_i() + tr.get_x_i(), ellipse.center.get_y_i() + tr.get_y_i(), ellipse.color);
                self.put_pixel(ellipse.center.get_x_i() + tl.get_x_i(), ellipse.center.get_y_i() + tl.get_y_i(), ellipse.color);
                self.put_pixel(ellipse.center.get_x_i() + br.get_x_i(), ellipse.center.get_y_i() + br.get_y_i(), ellipse.color);
                self.put_pixel(ellipse.center.get_x_i() + bl.get_x_i(), ellipse.center.get_y_i() + bl.get_y_i(), ellipse.color);
            }
        }
        
    }
    // pub fn put_sprite(&mut self, sprite: &RGBACanvas, sprite_com: Coord, )
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

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
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
}

pub struct Line {
    pub start: Coord,
    pub end: Coord,
    pub color: RGBAColor,
}

impl Line {
    pub fn new(start: Coord, end: Coord, color: RGBAColor) -> Line {
        return Line{
            start,
            end,
            color,
        };
    } 
}

/* pub struct Dot {
    pub center: Coord,
    pub color: RGBAColor,
} */

#[derive(Clone, Copy)]
pub struct Ellipse {
    pub center: Coord,
    pub hor_axis: f64,
    pub ver_axis: f64,
    pub angle: f64,
    pub color: RGBAColor,
}

impl Ellipse {
    pub fn new(center: Coord, hor_axis: f64, ver_axis: f64, color: RGBAColor) -> Ellipse {
        if hor_axis <= 0.0 { panic!("Horisontal axis for Ellipse should be positive number!"); }
        if ver_axis <= 0.0 { panic!("Vertical axis for Ellipse should be positive number!"); }

        return Ellipse{center, hor_axis, ver_axis, angle: 0.0, color};
    }

    pub fn turn(&mut self, angle: f64) {
        let mut new_angle: f64 = self.angle + angle;

        while new_angle > std::f64::consts::TAU {
            new_angle -= std::f64::consts::TAU;
        }

        while new_angle < 0.0 {
            new_angle += std::f64::consts::TAU;
        }

        self.angle = new_angle;
    }
}