use crate::{common_structs::{Coord, RGBAColor}, rgba_canvas::RGBACanvas};

#[derive(Copy, Clone)]
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

    pub fn draw_line_p(&self, canvas: &mut RGBACanvas) {
        // draw parametric line
        
        let x_s: i32 = self.start.get_x_i();
        let y_s: i32 = self.start.get_y_i();
        
        let delta_x: f64 = self.end.x() - self.start.x();
        let delta_y: f64 = self.end.y() - self.start.y();
        let mod_d_x: f64;
        let mod_d_y: f64;
        let mut t: f64;
    
        let step_x: f64;
        let step_y: f64;
    
        let mut x: i32;
        let mut y: i32;
    
        if delta_x < 0.0 { mod_d_x = - delta_x; } else { mod_d_x = delta_x; }
        if delta_y < 0.0 { mod_d_y = - delta_y; } else { mod_d_y = delta_y; }

        if mod_d_x >= mod_d_y {
            t = mod_d_x;
        } else {
            t = mod_d_y;
        }

        if t < 1.0 { t = 1.0; } 
    
        step_x = delta_x / t;
        step_y = delta_y / t;
    
        for t in 0..(t as usize + 1) {
            x = x_s + (t as f64 * step_x) as i32;
            y = y_s + (t as f64 * step_y) as i32;

            canvas.put_pixel(x, y, self.color);
        }
    }

    pub fn draw(&self, canvas: &mut RGBACanvas) {
        // this should draw line using line equation
        let a: f64;
        let b: f64;

        let mut x: f64;
        let mut y: f64;

        let incr_x: f64;
        let incr_y: f64;

        let delta_x: i32;
        let delta_y: i32;

        if self.end.x() - self.start.x() >= 0.0 {
            delta_x = (self.end.x() - self.start.x()) as i32;
            incr_x = 1.0;
        } else {
            delta_x = (self.start.x() - self.end.x()) as i32;
            incr_x = -1.0;
        }

        if self.end.y() - self.start.y() >= 0.0 {
            delta_y = (self.end.y() - self.start.y()) as i32;
            incr_y = 1.0;
        } else {
            delta_y = (self.start.y() - self.end.y()) as i32;
            incr_y = -1.0
        }
        
        if delta_x != 0 {

            a = (self.end.y() - self.start.y()) / (self.end.x() - self.start.x());
            b = self.start.y() - a * self.start.x();

            if a >= -1.0 && a <= 1.0 {
                for i in 0..(delta_x + 1) {
                    x = self.start.x() + incr_x * i as f64;
                    y = a * x + b;

                    canvas.put_pixel(x as i32, y as i32, self.color);
                }
            } else {
                for j in 0..(delta_y + 1) {
                    y = self.start.y() + incr_y * j as f64;
                    x = (y - b) / a;

                    canvas.put_pixel(x as i32, y as i32, self.color);
                }
            }

        } else {

            x = self.start.x();

            for j in 0..(delta_y + 1) {
                y = self.start.y() + incr_y * j as f64;

                canvas.put_pixel(x as i32, y as i32, self.color);
            }

        }
    }

    pub fn draw_smooth(&self, canvas: &mut RGBACanvas) {
        // this should draw smooth line using line equation(?)
    }

    pub fn intersection(&self, other: Line) -> Option<Coord> {
        // this method will return coordinates of intersection point if there is an intersection between lines

        // placeholder
        let is_intersecting: bool = false;

        
        
        if is_intersecting {
            return Some(Coord::new(0.0, 0.0));
        } else {
            return None;
        }
    }

    pub fn is_vert(&self) -> bool {
        if self.end.x() - self.start.y() == 0.0 {
            return true;
        } else {
            return false;
        }
    }
}