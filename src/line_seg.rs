use crate::{common_structs::{Coord, RGBAColor, AlignedBox, Angle, Dot, Marker, Palette}, rgba_canvas::RGBACanvas, vector_2d::Vector2D, linear_texture::{TransType, TextType}};

#[derive(Copy, Clone)]
pub struct LineSeg {
    pub start: Coord,
    pub end: Coord,
    pub color: RGBAColor,
}

impl LineSeg {
    pub fn new(start: Coord, end: Coord, color: RGBAColor) -> LineSeg {
        return LineSeg{
            start,
            end,
            color,
        };
    }

    pub fn draw_line_p(&self, canvas: &mut RGBACanvas) {
        // draw parametric line
        
        let x_s: i32 = self.start.get_x_i();
        let y_s: i32 = self.start.get_y_i();
        
        let delta_x: f32 = self.end.x() - self.start.x();
        let delta_y: f32 = self.end.y() - self.start.y();
        let mod_d_x: f32;
        let mod_d_y: f32;
        let mut t: f32;
    
        let step_x: f32;
        let step_y: f32;
    
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
            x = x_s + (t as f32 * step_x) as i32;
            y = y_s + (t as f32 * step_y) as i32;

            canvas.put_pixel(x, y, self.color);
        }
    }

    pub fn draw_line_p_v(&self, canvas: &mut RGBACanvas) {
        // interim check for Vector2D::draw_simple()
        let mut new_vec: Vector2D = Vector2D::from_line_seg(self);

        new_vec.texture.add_periodic_texture(
            RGBAColor::new_p(Palette::Blue),
            40.0,
            0.3,
            TextType::Step(0.2),
        );

        new_vec.texture.add_edges(
            RGBAColor::new_p(Palette::White),
            5.0,
            TransType::Step,
        );

        new_vec.draw_simple(canvas);
    }

    pub fn draw(&self, canvas: &mut RGBACanvas) {
        // this should draw line using line equation
        let a: f32;
        let b: f32;

        let mut x: f32;
        let mut y: f32;

        let incr_x: f32;
        let incr_y: f32;

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
                    x = self.start.x() + incr_x * i as f32;
                    y = a * x + b;

                    canvas.put_pixel(x as i32, y as i32, self.color);
                }
            } else {
                for j in 0..(delta_y + 1) {
                    y = self.start.y() + incr_y * j as f32;
                    x = (y - b) / a;

                    canvas.put_pixel(x as i32, y as i32, self.color);
                }
            }

        } else {

            x = self.start.x();

            for j in 0..(delta_y + 1) {
                y = self.start.y() + incr_y * j as f32;

                canvas.put_pixel(x as i32, y as i32, self.color);
            }

        }
    }

    pub fn draw_smooth(&self, canvas: &mut RGBACanvas) {
        // this should draw smooth line using line equation(?)
    }

    pub fn get_axis_aligned_box(&self) -> AlignedBox {
        let x_0: f32;
        let y_0: f32;
        let x_1: f32;
        let y_1: f32;

        if self.start.x() < self.end.x() {
            x_0 = self.start.x();
            x_1 = self.end.x();
        } else if self.start.x() == self.end.x() {
            x_0 = self.start.x() - 0.1;
            x_1 = self.end.x() + 0.1;
        } else {
            x_0 = self.end.x();
            x_1 = self.start.x();
        }

        if self.start.y() < self.end.y() {
            y_0 = self.start.y();
            y_1 = self.end.y();
        } else if self.start.y() == self.end.y() {
            y_0 = self.start.y() - 0.1;
            y_1 = self.end.y() + 0.1;
        } else {
            y_0 = self.end.y();
            y_1 = self.start.y();
        }

        return AlignedBox{x_0, x_1, y_0, y_1};
    }

    pub fn intersection(&self, other: &LineSeg) -> Option<Dot> {
        // this method will return coordinates of intersection point if there is an intersection between lines

        // if lines on top of each other, they share a continuous line segment, and
        //  there is no single intersection point -> take this as "no intersection", they are "just touching"

        let mut is_intersecting: bool;
        let mut x_i: f32 = 0.0;
        let mut y_i: f32 = 0.0;

        let coeff_1: LineCoeff = LineCoeff::get(self);
        let coeff_2: LineCoeff = LineCoeff::get(other);

        let self_box: AlignedBox = self.get_axis_aligned_box();
        let other_box: AlignedBox = other.get_axis_aligned_box();
        let intersect_box: AlignedBox;

        let intersect_area: Option<AlignedBox> = self_box.box_intersection(&other_box);

        if intersect_area.is_some() {
            intersect_box = intersect_area.unwrap();
            is_intersecting = true;

            if coeff_1.v {
                if coeff_2.v {
                    // both are vertical
                    // lines share segment --> "touching", not intersecting

                    is_intersecting = false;
                } else {
                    // only self is vertical

                    x_i = self.start.x();
                    y_i = coeff_2.a * x_i + coeff_2.b;
                }
            } else {
                if coeff_2.v {
                    // only other is vertical

                    x_i = other.start.x();
                    y_i = coeff_1.a * x_i + coeff_1.b;
                } else {
                    // none are vertical

                    if coeff_1.a == coeff_2.a && coeff_1.b == coeff_2.b {
                        // segments of the same line
                        // same as lines on top of each other
                        // lines share segment --> "touching", not intersecting

                        is_intersecting = false;
                    } else {
                        // lines do cross at specific point

                        x_i = (coeff_2.b - coeff_1.b) / (coeff_1.a - coeff_2.a);
                        y_i = x_i * coeff_1.a + coeff_1.b;
                    }
                }
            }

            if is_intersecting {
                if  x_i > intersect_box.x_0 - 0.1 &&
                    x_i < intersect_box.x_1 + 0.1 &&
                    y_i > intersect_box.y_0 - 0.1 &&
                    y_i < intersect_box.y_1 + 0.1
                {
                    // println!("intersection at point x: {:.2}, y: {:.2}", x_i, y_i);
                    return Some(Dot::new(Coord::new(x_i, y_i), other.color, Marker::Square(0)));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn new_rot_offset_line(&self, alpha: Angle, offset: Coord) -> LineSeg {
        // rotation is about starting point

        let rot_endpoint: Coord = Coord::new(
            self.end.x() - self.start.x(),
            self.end.y() - self.start.y(),
        ).new_rotated(alpha);

        return LineSeg::new(
            self.start.new_offset(offset),
            rot_endpoint.new_offset(self.start).new_offset(offset),
            self.color,
        );
    }

    pub fn get_length(&self) -> f32 {
        return f32::sqrt((self.end.x() - self.start.x()) * (self.end.x() - self.start.x()) + 
                            (self.end.y() - self.start.y()) * (self.end.y() - self.start.y()));
    }
}

struct LineCoeff {
    a: f32,
    b: f32,
    v: bool,
}

impl LineCoeff {
    pub fn get(line: &LineSeg) -> LineCoeff {
        let v: bool = line.end.x() - line.start.x() == 0.0;
        let a: f32;
        let b: f32;

        if v {
            a = line.start.x();
            b = 0.0;
        } else {
            a = (line.end.y() - line.start.y()) / (line.end.x() - line.start.x());
            b = line.start.y() - a * line.start.x();
        }

        return LineCoeff{a, b, v};
    }
}