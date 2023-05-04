use crate::common_structs::{Coord, RGBAColor, RGBACanvas};

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
}