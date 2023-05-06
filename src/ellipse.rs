// ellipse struct with its functions (methods)

use crate::{common_structs::{Coord, Angle, RGBAColor}, rgba_canvas::RGBACanvas};

#[derive(Clone, Copy)]
pub struct Ellipse {
    pub center: Coord,
    pivot: Coord, // by default same as center
    hor_axis: f64,
    ver_axis: f64,
    pub angle: Angle,
    pub color: RGBAColor,
}

impl Ellipse {
    pub fn new(center: Coord, hor_axis: f64, ver_axis: f64, color: RGBAColor) -> Ellipse {
        if hor_axis <= 0.0 { panic!("Horisontal axis for Ellipse should be positive number!"); }
        if ver_axis <= 0.0 { panic!("Vertical axis for Ellipse should be positive number!"); }

        return Ellipse{center, pivot: center, hor_axis, ver_axis, angle: Angle::new(), color};
    }

    pub fn draw_ellipse_raster(&self, canvas: &mut RGBACanvas, is_filled: bool, edge_width: f64) {
        // treat ellipse as all the dots that have sum of distances from foci constant
        let mut foc_1: Coord = Coord::new(0.0, 0.0);
        let mut foc_2: Coord = Coord::new(0.0, 0.0);
        let mut smooth_color: RGBAColor = self.color;

        let long_semi_axis: f64;
        let foc_distance: f64; // distance from center to focus
        let mut sum_dist_from_foci: f64;

        let mut x: f64;
        let mut y: f64;

        let sin_alpha: f64 = f64::sin(self.angle.get_value());
        let cos_alpha: f64 = f64::cos(self.angle.get_value());

        if self.hor_axis >= self.ver_axis {
            long_semi_axis = self.hor_axis;

            foc_distance = f64::sqrt(long_semi_axis * long_semi_axis - self.ver_axis * self.ver_axis);

            foc_1.set_x(foc_distance);
            foc_2.set_x(-foc_distance);
        } else {
            long_semi_axis = self.ver_axis;

            foc_distance = f64::sqrt(long_semi_axis * long_semi_axis - self.hor_axis * self.hor_axis);

            foc_1.set_y(foc_distance);
            foc_2.set_y(-foc_distance);
        }

        // turn
        foc_1 = Coord::new(foc_1.x() * cos_alpha - foc_1.y() * sin_alpha, foc_1.x() * sin_alpha + foc_1.y() * cos_alpha);
        foc_2 = Coord::new(foc_2.x() * cos_alpha - foc_2.y() * sin_alpha, foc_2.x() * sin_alpha + foc_2.y() * cos_alpha);

        // translate
        foc_1 = Coord::new(foc_1.x() + self.center.x(), foc_1.y() + self.center.y());
        foc_2 = Coord::new(foc_2.x() + self.center.x(), foc_2.y() + self.center.y());

        // fill pixels inside ellipse
        for j in (self.center.get_y_i()-(long_semi_axis as i32 + edge_width as i32))..(self.center.get_y_i()+(long_semi_axis as i32 + edge_width as i32 + 1)) {
            for i in (self.center.get_x_i()-(long_semi_axis as i32 + edge_width as i32))..(self.center.get_x_i()+(long_semi_axis as i32 + edge_width as i32 + 1)) {
                x = i as f64;
                y = j as f64;

                sum_dist_from_foci =
                    f64::sqrt((x - foc_1.x()) * (x - foc_1.x()) + (y - foc_1.y()) * (y - foc_1.y())) +
                    f64::sqrt((x - foc_2.x()) * (x - foc_2.x()) + (y - foc_2.y()) * (y - foc_2.y()));


                if is_filled {
                    if sum_dist_from_foci <= long_semi_axis * 2.0 {

                        canvas.put_pixel(i, j, self.color);
    
                    } else if sum_dist_from_foci <= long_semi_axis * 2.0 + edge_width {
                        // smooth edge

                        smooth_color.a = ((edge_width - (sum_dist_from_foci - long_semi_axis * 2.0)) * 255.0 / edge_width) as u8;
    
                        canvas.put_pixel(i, j, smooth_color);
                    }
                } else {
                    if sum_dist_from_foci > long_semi_axis * 2.0 - edge_width && sum_dist_from_foci <= long_semi_axis * 2.0 {
                        smooth_color.a = ((edge_width - (long_semi_axis * 2.0 - sum_dist_from_foci)) * 255.0 / edge_width) as u8;

                        canvas.put_pixel(i, j, smooth_color);
    
                    } else if sum_dist_from_foci <= long_semi_axis * 2.0 + edge_width && sum_dist_from_foci > long_semi_axis * 2.0 {
                        smooth_color = self.color;
                        smooth_color.a = ((edge_width - (sum_dist_from_foci - long_semi_axis * 2.0)) * 255.0 / edge_width) as u8;
    
                        canvas.put_pixel(i, j, smooth_color);
                    }
                }
            }
        }
    }
}