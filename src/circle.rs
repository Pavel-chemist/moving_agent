// Circle struct with all its implementations

use crate::common_structs::{RGBColor, RGBCanvas};

#[derive(Clone)]
pub struct Circle {
    pub name: String,
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub radius: f64,
    border_width: f64,
    body_color: RGBColor,
    border_color: RGBColor,
    pub mass: f64,
    is_selected: bool,
}

impl Circle {
    pub fn new(
        name: String,
        x_pos: f64,
        y_pos: f64,
        x_vel: f64,
        y_vel: f64,
        radius: f64,
        mut border_width: f64,
        mass: f64,
        body_color: RGBColor,
        border_color: RGBColor,
    ) -> Circle {
        if border_width > radius {
            border_width = radius;
        }

        return Circle {
            name,
            x_pos,
            y_pos,
            x_vel,
            y_vel,
            radius,
            border_width,
            body_color,
            border_color,
            mass,
            is_selected: false,
        }
    }

    pub fn clone(&self) -> Circle {
        return Circle {
            name: String::from(&self.name),
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            x_vel: self.x_vel,
            y_vel: self.y_vel,
            radius: self.radius,
            border_width: self.border_width,
            body_color: self.body_color,
            border_color: self.border_color,
            mass: self.mass,
            is_selected: false,
        }
    }

    pub fn move_circle(&mut self, x_range_start: f64, x_range_end: f64, y_range_start: f64, y_range_end: f64) {
        let vel_sq: f64 = self.x_vel * self.x_vel + self.y_vel * self.y_vel;
        
        if self.x_pos + self.x_vel <= x_range_start + self.radius {

            // when touching start of range, calculate reflection    
            self.x_pos = 2.0 * self.radius - self.x_pos - self.x_vel;
            self.x_vel = -self.x_vel;

        } else if self.x_pos + self.x_vel >= x_range_end - self.radius {

            // when touching end of range, calculate reflection
            self.x_pos = 2.0 * x_range_end - 2.0 * self.radius - self.x_pos - self.x_vel;
            self.x_vel = -self.x_vel;

        } else {
            
            // when in range, just proceed as always
            self.x_pos += self.x_vel;
        }

    
        if self.y_pos + self.y_vel <= y_range_start + self.radius {

            // when touching start of range, calculate reflection    
            self.y_pos = 2.0 * self.radius - self.y_pos - self.y_vel;
            self.y_vel = -self.y_vel;

        } else if self.y_pos + self.y_vel >= y_range_end - self.radius {

            // when touching end of range, calculate reflection
            self.y_pos = 2.0 * y_range_end - 2.0 * self.radius - self.y_pos - self.y_vel;
            self.y_vel = -self.y_vel;

        } else {
            
            // when in range, just proceed as always
            self.y_pos += self.y_vel;
        }

        if vel_sq > 100.0 {
            self.x_vel = self.x_vel * 0.99;
            self.y_vel = self.y_vel * 0.99;
        }
    }

    pub fn collide_with_other_circles(&mut self, circles_array: &Vec<Circle>, own_index: usize) {
        let mut distance_squared: f64;
        let mut sum_radii_squared: f64;

        let mut rel_x: f64;
        let mut rel_y: f64;
        let mut rel_x_t: f64;
        let mut rel_y_t: f64;
        let mut rel_v_x: f64;
        let mut rel_v_y: f64;
        let mut abs_rel_vel: f64;
        let mut a: f64;
        let mut b: f64;
        let mut c: f64;
        let mut t: f64;
        let mut t1: f64;
        let mut t2: f64;
        let mut sin_phi: f64;
        let mut cos_phi: f64;
        let mut new_v_x: f64;
        let mut new_v_y: f64;

        let old_m: f64 = self.mass * f64::sqrt(self.x_vel * self.x_vel + self.y_vel * self.y_vel);
        let mut new_m: f64;

        for i in 0..circles_array.len() {
            if i != own_index {
                distance_squared = (circles_array[i].x_pos - self.x_pos) * (circles_array[i].x_pos - self.x_pos) + (circles_array[i].y_pos - self.y_pos) * (circles_array[i].y_pos - self.y_pos);

                sum_radii_squared = (circles_array[i].radius +  self.radius) * (circles_array[i].radius +  self.radius);
    
                if distance_squared < sum_radii_squared {

                    // these are relative velocity components of collider  
                    rel_v_x = circles_array[i].x_vel - self.x_vel;
                    rel_v_y = circles_array[i].y_vel - self.y_vel;

                    // magnitude of collider's relative velocity
                    abs_rel_vel = f64::sqrt(rel_v_x * rel_v_x + rel_v_y * rel_v_y);

                    // relative positions of collider at t0 and t1
                    rel_x = circles_array[i].x_pos - self.x_pos;
                    rel_y = circles_array[i].y_pos - self.y_pos;
 
                    if abs_rel_vel != 0.0 {
                        // coefficients for calculating the precize time of collision
                        a = abs_rel_vel;
                        b = 2.0 * (rel_v_x * rel_x + rel_v_y * rel_y);
                        c = rel_x * rel_x + rel_y * rel_y - sum_radii_squared;

                        t1 = (-b + f64::sqrt(b * b - 4.0 * a * c))/(2.0 * a);
                        t2 = (-b - f64::sqrt(b * b - 4.0 * a * c))/(2.0 * a);

                        t = t2;

                        // relative position at impact
                        rel_x_t = rel_x + t * rel_v_x;
                        rel_y_t = rel_y + t * rel_v_y;

                        sin_phi = rel_y_t / f64::sqrt(sum_radii_squared);
                        cos_phi = rel_x_t / f64::sqrt(sum_radii_squared);
    
                        new_v_x = cos_phi * (rel_v_x * cos_phi + rel_v_y * sin_phi);
                        new_v_y = sin_phi * (rel_v_x * cos_phi + rel_v_y * sin_phi);
    
                        if t > -1.0 {
                            self.x_pos = self.x_pos + self.x_vel * t - new_v_x * t;
                            self.y_pos = self.y_pos + self.y_vel * t - new_v_y * t;

                            self.x_vel += new_v_x * circles_array[i].mass / self.mass;
                            self.y_vel += new_v_y * circles_array[i].mass / self.mass;
                        } else {
                            self.x_pos = self.x_pos - self.x_vel;
                            self.y_pos = self.y_pos - self.y_vel;

                            self.x_vel += new_v_x * circles_array[i].mass / self.mass;
                            self.y_vel += new_v_y * circles_array[i].mass / self.mass;
                        }

                        new_m = self.mass * f64::sqrt(self.x_vel * self.x_vel + self.y_vel * self.y_vel);

                        if t < -1.0 {
                            println!("#{:>3} -> #{:>3}: t= {:>8.3}; old_m= {:>5.3}; new_m= {:>5.3}", own_index + 1, i + 1, t, old_m, new_m);
                        }
                    }
                }
            }
        }
    }



    pub fn check_on_top(circle: &Circle, circles_array: &Vec<Circle>, own_index: usize) -> bool {
        let mut is_on_top: bool = false;
        let mut distance_squared: f64;
        let mut sum_radii_squared: f64;

        for i in 0..circles_array.len() {
            if i != own_index {
                distance_squared = (circles_array[i].x_pos - circle.x_pos) * (circles_array[i].x_pos - circle.x_pos) + (circles_array[i].y_pos - circle.y_pos) * (circles_array[i].y_pos - circle.y_pos);

                sum_radii_squared = (circles_array[i].radius +  circle.radius) * (circles_array[i].radius +  circle.radius);
    
                if distance_squared < sum_radii_squared {
                    is_on_top = true;
    
                    break;
                }
            }
        }

        return is_on_top;
    }

    pub fn accelerate_to_position(&mut self, new_x: f64, new_y: f64) {
        self.x_vel = new_x - self.x_pos;
        self.y_vel = new_y - self.y_pos;

        self.x_pos = new_x;
        self.y_pos = new_y;
    }

    pub fn put_on_canvas_smoothed(&self, canvas: &mut RGBCanvas) {
        if self.x_pos >= 0.0 - self.radius
        && self.x_pos < canvas.width + self.radius
        && self.y_pos >= 0.0 - self.radius
        && self.y_pos < canvas.height + self.radius
        {
            let width: i32 = canvas.width as i32;
            // let x: i32 = self.x_pos as i32;
            // let y: i32 = self.y_pos as i32;
            let box_lx: i32 = if self.x_pos > self.radius {(self.x_pos - self.radius) as i32} else {0};
            let box_hx: i32 = (self.x_pos + self.radius + 2.0) as i32;
            let box_ly: i32 = if self.y_pos > self.radius {(self.y_pos - self.radius) as i32} else {0};
            let box_hy: i32 = (self.y_pos + self.radius + 2.0) as i32;

            let mut distance: f64;

            let inner_radius: f64 = self.radius - self.border_width;
            
            let mut x_f: f64;
            let mut y_f: f64;

            let mut d: f64; //distance from the edge, must be between 0.0 and 1.0

            for j in box_ly..box_hy {
                for i in box_lx..box_hx {
                    if i >= 0 && i < canvas.width as i32&& j >= 0 && j < canvas.height as i32 {
                        x_f = i as f64;
                        y_f = j as f64;

                        distance = f64::sqrt((self.x_pos - x_f) * (self.x_pos - x_f) + (self.y_pos - y_f) * (self.y_pos - y_f));

                        if distance <= inner_radius {
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = self.body_color.r;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = self.body_color.g;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = self.body_color.b;
                        } else if distance <= inner_radius + 1.0 {
                            d = distance - inner_radius;
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = ((self.body_color.r as f64) * (1.0 - d) + (self.border_color.r as f64) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = ((self.body_color.g as f64) * (1.0 - d) + (self.border_color.g as f64) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = ((self.body_color.b as f64) * (1.0 - d) + (self.border_color.b as f64) * d) as u8;
                        } else if distance <= self.radius {
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = self.border_color.r;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = self.border_color.g;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = self.border_color.b;
                        } else if distance <= self.radius + 1.0 {
                            d = distance - self.radius;
                            canvas.data[(width * j * 3 + i * 3 + 0) as usize] = ((self.border_color.r as f64) * (1.0 - d) + (canvas.data[(width * j * 3 + i * 3 + 0) as usize] as f64) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 1) as usize] = ((self.border_color.g as f64) * (1.0 - d) + (canvas.data[(width * j * 3 + i * 3 + 1) as usize] as f64) * d) as u8;
                            canvas.data[(width * j * 3 + i * 3 + 2) as usize] = ((self.border_color.b as f64) * (1.0 - d) + (canvas.data[(width * j * 3 + i * 3 + 2) as usize] as f64) * d) as u8;
                        }
                    }
                }
            }
        }
    }
}