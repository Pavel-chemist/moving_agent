use rand::{Rng, rngs::ThreadRng};

use crate::rgba_canvas::RGBACanvas;

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

    pub fn new_rand() -> RGBAColor {
        let mut rng: ThreadRng = rand::thread_rng();

        return RGBAColor{r: rng.gen_range(128..255), g: rng.gen_range(128..255), b: rng.gen_range(128..255), a: 255};
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

    pub fn new_scaled(&self, scaling_factor: f64) -> RGBAColor {
        let new_color: RGBAColor;

        if scaling_factor <= 0.0 {
            new_color = RGBAColor::new_black();
        } else if scaling_factor > 0.0 && scaling_factor < 1.0 {
            new_color = RGBAColor::new_rgb(
                (self.r as f64 * scaling_factor) as u8,
                (self.g as f64 * scaling_factor) as u8,
                (self.b as f64 * scaling_factor) as u8,
            );
        } else {
            new_color = self.clone();
        }
        
        return new_color;
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

    pub fn new_rotated(&self, alpha: Angle) -> Coord {
        return Coord::new(
            f64::cos(alpha.a) * self.x - f64::sin(alpha.a) * self.y,
            f64::sin(alpha.a) * self.x + f64::cos(alpha.a) * self.y,
        );
    }

    pub fn new_offset(&self, offset: Coord) -> Coord {
        return Coord::new(
            self.x + offset.x,
            self.y + offset.y,
        );
    }
}

#[derive(Clone, Copy)]
pub enum Marker {
    Square(i32),
    Disc(i32),
}

#[derive(Clone, Copy)]
pub struct Dot {
    pub coord: Coord,
    pub color: RGBAColor,
    marker: Marker,
}

impl Dot {
    pub fn new(coord: Coord, color: RGBAColor, marker: Marker) -> Dot {
        return Dot{coord, color, marker};
    }

    pub fn new_rot_offset(&self, alpha: Angle, offset: Coord) -> Dot {
        return Dot::new(
            self.coord.new_rotated(alpha).new_offset(offset),
            self.color,
            self.marker,
          );
    }

    pub fn draw(&self, canvas: &mut RGBACanvas) {
        match self.marker {
            Marker::Square(size) => {
                canvas.put_square(self.coord.get_x_i(), self.coord.get_y_i(), size, self.color);
            }
            Marker::Disc(radius) => {
                canvas.put_disc(self.coord.get_x_i(), self.coord.get_y_i(), radius, self.color);
            }
        }
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

    pub fn new_rad(radians: f64) -> Angle {
        return Angle{ a: radians };
    }

    pub fn new_deg(degrees: f64) -> Angle {
        return Angle{ a: degrees * std::f64::consts::TAU / 360.0 };
    }

    pub fn turn(&mut self, alpha: Angle) {

        self.a += alpha.a;
        self.normalize();
    }

    pub fn turn_rad(&mut self, value: f64) {

        self.a += value;
        self.normalize();
    }

    pub fn turn_deg(&mut self, degrees: f64) {

        self.a += degrees * std::f64::consts::TAU / 360.0;
        self.normalize();
    }

    pub fn get_rad(&self) -> f64 {
        return self.a;
    }

    pub fn get_deg(&self) -> f64 {
        return self.a * 360.0 / std::f64::consts::TAU;
    }

    fn normalize(&mut self) {
        // ensure that angle is in range [0..TAU) radians

        if !(self.a >= 0.0 && self.a < std::f64::consts::TAU) {

            self.a = self.a - std::f64::consts::TAU * (self.a / std::f64::consts::TAU).trunc();
        }
    }
}

#[derive(Clone, Copy)]
struct LineSeg {
    x_0: f64,
    x_1: f64,
}

impl LineSeg {
    pub fn linear_intersection(a_0: f64, a_1: f64, b_0: f64, b_1: f64) -> Option<LineSeg> {
        let is_intersected: bool;
        let x_0: f64;
        let x_1: f64;

        if a_1 < b_0 || a_0 > b_1 {
            // no intersection
            is_intersected = false;
            x_0 = 0.0;
            x_1 = 0.0;
        } else {
            is_intersected = true;

            if a_0 < b_0 {
                x_0 = b_0;

                if a_1 < b_1 {
                    x_1 = a_1;
                } else {
                    x_1 = b_1;
                }
            } else {
                x_0 = a_0;

                if a_1 < b_1 {
                    x_1 = a_1;
                } else {
                    x_1 = b_1;
                }
            }
        }

        if is_intersected {
            return Some(LineSeg {x_0, x_1});
        } else {
            return None;
        }
    } 
}

#[derive(Clone, Copy)]
pub struct AlignedBox {
    pub x_0: f64,
    pub y_0: f64,
    pub x_1: f64,
    pub y_1: f64,
}

impl  AlignedBox {
    // pub fn new()
    pub fn box_intersection(&self, tested_box: &AlignedBox) -> Option<AlignedBox> {

        let x_intersection: Option<LineSeg> = LineSeg::linear_intersection(
            self.x_0,
            self.x_1,
            tested_box.x_0,
            tested_box.x_1,
        );

        let y_intersection: Option<LineSeg> = LineSeg::linear_intersection(
            self.y_0,
            self.y_1,
            tested_box.y_0,
            tested_box.y_1,
        );

        if x_intersection.is_none() || y_intersection.is_none() {
            return None;
        } else {
            let intersection_box = AlignedBox {
                x_0: x_intersection.unwrap().x_0,
                y_0: y_intersection.unwrap().x_0,
                x_1: x_intersection.unwrap().x_1,
                y_1: y_intersection.unwrap().x_1,
            };

            return Some(intersection_box);
        }
    }
}