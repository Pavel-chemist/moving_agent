use std::collections::btree_set::Intersection;

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