// here, the object describing the state

use rand::{random, Rng};

use crate::{circle::Circle, common_structs::{RGBCanvas, RGBColor}};


pub struct State {
    pub width: f64, //world width
    pub height: f64, //world height
    background: Vec<u8>, //array containing rgb values for background image
    pub circles: Vec<Circle>,
    pub selected_circle_index: usize,
    pub has_selected_circle: bool,
    pub global_time: usize,
}

impl State {
    pub fn new(width: i32, height: i32) -> State {
        return State {
            width: width as f64,
            height: height as f64,
            background: State::create_background(width, height),
            circles: Vec::<Circle>::new(),
            selected_circle_index: 0,
            has_selected_circle: false,
            global_time: 0,
        };
    }

    pub fn get_rendered_view(&self) -> RGBCanvas {
        let mut rendered_scene = RGBCanvas::new(self.width, self.height);
        
        for i in 0..self.background.len() {
            rendered_scene.data[i] = self.background[i];
        }

        for i in 0..self.circles.len() {
            // self.circles[i].put_on_canvas(&mut rendered_scene);
            self.circles[i].put_on_canvas_smoothed(&mut rendered_scene);
        }

        return rendered_scene;
    }

/////////////////////////////////////////////////////////
    
    pub fn add_circle(&mut self, circle: Circle) {
        // ensure that new circle is not on top of another
        let is_on_top = Circle::check_on_top(&circle, &self.circles, 999999);

        if !is_on_top {
            self.circles.push(circle);
        } else {
            println!("trying to put circle on top of another one");
        }
    }

    pub fn add_random_circle_at_coords(&mut self, x: i32, y: i32, x_vel: f64, y_vel: f64) {
        let mut rng = rand::thread_rng();

        let mut new_circle: Circle = Circle::new(
            String::from("Circle"), 
            x as f64,
            y as f64,
            x_vel,
            y_vel,
            rng.gen_range(15.0..25.0),
            rng.gen_range(3.0..13.0),
            1.0,
            RGBColor {
                r: random(),
                g: random(),
                b: random(),
            },
            RGBColor {
                r: random(),
                g: random(),
                b: random(),
            },
        );

        new_circle.mass = new_circle.radius * new_circle.radius;

        self.add_circle(new_circle);
    }

    pub fn select_circle(&mut self, x: i32, y: i32) {
        let x_pos: f64 = x as f64;
        let y_pos: f64 = y as f64;

        let mut is_selected: bool = false;
        let mut selected_index: usize = 0;
        
        for (index, circle) in self.circles.iter().enumerate().rev() {
            if (x_pos - circle.x_pos) * (x_pos - circle.x_pos) < circle.radius * circle.radius
            && (y_pos - circle.y_pos) * (y_pos - circle.y_pos) < circle.radius * circle.radius
            {
                selected_index = index;
                is_selected = true;

                println!("Circle with index {} is selected", index);
                break;
            }
        }

        if is_selected {
            println!("Circle was selected");
            let selected_circle = self.circles.remove(selected_index);
            self.circles.push(selected_circle);
            self.selected_circle_index = self.circles.len() - 1;
        } else {
            println!("No selection");
        }
        
        self.has_selected_circle = is_selected;
    }

    pub fn remove_circle(&mut self) {
        let mut new_circles_array: Vec<Circle> = Vec::new();

        if self.has_selected_circle {
            for i in 0..self.circles.len() {
                if i != self.selected_circle_index {
                    new_circles_array.push(self.circles[i].clone());
                }
            }

            self.circles = new_circles_array;
        }
        
        self.has_selected_circle = false;
    }

    pub fn replace_background(&mut self, new_background: Vec<u8>) {
        if new_background.len() == self.background.len() {
            self.background = new_background;
        } else {
            panic!("Error replacing background: The new background should be the same size as old one!");
        }
    }

    pub fn progress_one_step(&mut self) {
        self.global_time += 1;

        for i in 0..self.circles.len() {
            self.circles[i].move_circle(
                0.0,
                self.width,
                0.0,
                self.height,
            );
        }

        self.enumerate_collided_pairs();

        let mut cloned_circles_array: Vec<Circle> = Vec::with_capacity(self.circles.len());

        for i in 0..self.circles.len() {
            cloned_circles_array.push(self.circles[i].clone());
        }

        for i in 0..self.circles.len() {
            self.circles[i].collide_with_other_circles(&cloned_circles_array, i);
        }
    }

    fn enumerate_collided_pairs(&self) {
        struct Pair {
            i: usize,
            j: usize,
        }
        // check for collision
        let mut distance_squared: f64;
        let mut sum_radii_squared: f64;
        let mut collided_pairs_list: Vec<Pair> = Vec::new();

        for j in 0..self.circles.len() {
            for i in j..self.circles.len() {
                if i != j {
                    distance_squared = 
                    (self.circles[i].x_pos - self.circles[j].x_pos) * (self.circles[i].x_pos - self.circles[j].x_pos) +
                    (self.circles[i].y_pos - self.circles[j].y_pos) * (self.circles[i].y_pos - self.circles[j].y_pos);
    
                    sum_radii_squared = 
                        (self.circles[i].radius +  self.circles[j].radius) * 
                        (self.circles[i].radius +  self.circles[j].radius);
    
                    if distance_squared < sum_radii_squared {
                        collided_pairs_list.push(Pair{i, j});
                    }
                }
            }
        }

        if collided_pairs_list.len() > 0 {
            print!("{:>5}: ", self.global_time);
            for i in 0..collided_pairs_list.len() {
                print!("{:>3} <->{:>3};", collided_pairs_list[i].i, collided_pairs_list[i].j);
            };
            println!("");
        }
    }

    pub fn get_total_momentum(&self) -> f64 {
        let mut total_momentum: f64 = 0.0;
        let mut single_circle_momentum: f64;

        for i in 0..self.circles.len() {
            single_circle_momentum = f64::sqrt(self.circles[i].x_vel * self.circles[i].x_vel + self.circles[i].y_vel * self.circles[i].y_vel) * self.circles[i].mass;

            total_momentum += single_circle_momentum;
        }

        return total_momentum;
    }

    fn create_background(width: i32, height: i32) -> Vec<u8>{
        let num_pix: usize = (width * height) as usize;
        let data_array: Vec<u8>;
    
        data_array = vec![0; num_pix * 3];
    
        return data_array;
    }
}