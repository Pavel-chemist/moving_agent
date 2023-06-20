// Main things to do:
// make a first-person view for agent +
// make main view scalable

/* line textures...

procedural:
- edge textures -> change brightness / color depending on distance to the line ends, are symmetric around middle
  -- can be used to enchance or reduce corner visibility +
  
- periodic -> anchored to the start of line, characterized by period and phase
  -- can be adjusted to seamlesly wrap around a polygon +
  
  
  
Moving Agent features to add (with no particular order):
  
  1) pan and zoom for top-view +
  2) move rendering (raster creation) of top-view and first-person view into separate threads
  3) add smooth transitions for agent movements
  4) add agent collisions with lines and polygons in world +
  5) make prettier agent +
  7) create dot display modes i.e. display only polygon vertices:
   -- 2d "wireframe",
   -- 2d "wireframe with transparent surfaces"
   -- 2d "occluded wireframe"
  8) add stereo modes (parallel-eye/cross-eye and anaglyph)
  9) world segmentation (this is to decrease computational load for collisions and renderings)
  11) investigate openGL api
  
   */

use std::{path::Path, fs::File, io::Read};

use agent::Agent;
use common_structs::{
    Coord,
    Angle,
    RGBAColor, Palette,
};
use fltk::{
    app::{self, App, MouseButton, MouseWheel},
    enums::{self, ColorDepth, FrameType, Event},
    image::RgbImage,
    prelude::*,
    *,
};


use rgba_canvas::RGBACanvas;
use shape::{Shape, WorldSetup};
use world::World;

use crate::shape::ShapeDescription;

mod common_structs;
mod vector_2d;
mod linear_texture;
mod rgba_canvas;
mod ellipse;
mod shape;
mod agent;
mod world;

const WIND_LABEL: &str = "Moving Agent";
const WIND_WIDTH: i32 = 1000;
const WIND_HEIGHT: i32 = 720;
const MENU_HEIGHT: i32 = 32;

#[derive(Clone)]
enum Message {
    Quit,
    MouseDown(i32, i32, MouseButton),
    MouseDrag(i32, i32),
    MouseMove(i32),
    MouseReleased(i32, i32, MouseButton),
    Tick,
    KeyPress(char),
}

#[derive(Debug)]
enum ViewMode {
    Top,
    FirstPerson,
}

fn main() {
    let mut view_mode: ViewMode = ViewMode::Top;

    let initialization_data: WorldSetup = get_init_data_from_file().unwrap();

    let mut world: World = world::World::new();
    add_shapes_to_world(&mut world, initialization_data.world_shapes);

    let mut agent: Agent = Agent::new(
        initialization_data.initial_coord,
        Angle::new_deg(initialization_data.initial_angle_deg),
        Angle::new_deg(initialization_data.agents_field_of_view_deg),
    );

    agent.update_visible_walls(world.walls.clone());

    let application: App = app::App::default();

    let (s, r) = app::channel();

    let mut mouse_x: i32 = -1;
    let mut mouse_dx: i32 = 0;

    let mut wind = window::Window::new(0, 0, WIND_WIDTH, WIND_HEIGHT, WIND_LABEL);

    let mut menu = menu::SysMenuBar::default().with_size(wind.width(), MENU_HEIGHT);
    menu.set_frame(enums::FrameType::FlatBox);
    menu.set_color(enums::Color::Light2);

    menu.add_emit(
        "&File/Quit\t",
        enums::Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Quit,
    );

    let mut top_view_frame = frame::Frame::default()
        .with_pos(
            0,
            MENU_HEIGHT,
        )
        .with_size(WIND_WIDTH, WIND_HEIGHT - MENU_HEIGHT);
    wind.end();
    wind.show();

    // intercept keyboard events on the window
    let key_interceptor_sender =s.clone();
    let mut chars_vec: Vec<char> = Vec::new();
    wind.handle(move |_, event| match event {
        Event::KeyDown => {
            match app::event_key().to_char() {
                Some(char) => {
                    // println!("Key pressed: {:?}", char);
                    key_interceptor_sender.send(Message::KeyPress(char));
                    chars_vec.push(char);
                }
                None => {
                    // nothing
                }
            };
            false
        }
        Event::KeyUp => {
            match app::event_key().to_char() {
                Some(char) => {
                    // key_interceptor_sender.send(Message::KeyPress(char));
                    // println!("Key released: {:?}", char);
                }
                None => {
                    // nothing
                }
            };
            false
        }
        Event::Move => {
            // let mut x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
            // let mut y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
            // let d_x = app::event_dx();
            // let d_y = app::event_dy();

            let current_x = app::event_x();


            // println!("d_x: {:?}; d_y: {:?}", d_x, d_y);

            // if x < 0 { x = 0 }
            // if y < 0 { y = 0 }

            // if x >= MAIN_IMAGE_WIDTH { x = MAIN_IMAGE_WIDTH - 1 }
            // if y >= MAIN_IMAGE_HEIGHT { y = MAIN_IMAGE_HEIGHT - 1 }

            key_interceptor_sender.send(Message::MouseMove(current_x));

            true
        }

        _ => false,
    });

    let callback_sender = s.clone();
    
    let callback = move |handle| {
        callback_sender.send(Message::Tick);
        
        app::repeat_timeout3(0.016667, handle);
    };
    

    let top_view_frame_handle_sender = s.clone();
    top_view_frame.handle(move |_, event: Event| {
        match event {
            Event::Push => {
                let x = app::event_x();
                let y = app::event_y() - MENU_HEIGHT;
                let button = app::event_mouse_button();
                top_view_frame_handle_sender.send(Message::MouseDown(x, y, button));
                true
            }
            Event::Drag => {
                let x = app::event_x();
                let y = app::event_y() - MENU_HEIGHT;
                if x >= 0 && x < WIND_WIDTH && y >= 0 && y < WIND_HEIGHT - MENU_HEIGHT {
                    top_view_frame_handle_sender.send(Message::MouseDrag(x, y));
                }
                true
            }
            Event::Released => {
                let x = app::event_x();
                let y = app::event_y() - MENU_HEIGHT;
                let button = app::event_mouse_button();
                top_view_frame_handle_sender.send(Message::MouseReleased(x, y, button));
                true
            }
            /* Event::Move => {
                let mut x = app::event_x();
                let mut y = app::event_y() - MENU_HEIGHT;
                let d_x = app::event_dx();
                let d_y = app::event_dy();

                // println!("d_x: {:?}; d_y: {:?}", d_x, d_y);

                if x < 0 { x = 0 }
                if y < 0 { y = 0 }

                if x >= MAIN_IMAGE_WIDTH { x = MAIN_IMAGE_WIDTH - 1 }
                if y >= MAIN_IMAGE_HEIGHT { y = MAIN_IMAGE_HEIGHT - 1 }

                top_view_frame_handle_sender.send(Message::MouseMove(x, y, d_x, d_y));

                true
            } */
            _ => false,
        }
    });

    

    app::add_timeout3(0.033, callback);

    while application.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => {
                    println!("quitting the app...");
                    fltk::app::quit();
                }
                Message::Tick => {
                    match view_mode {
                        ViewMode::FirstPerson => draw_fisrt_person_view(
                            &mut agent,
                            &mut top_view_frame,
                        ),
                        ViewMode::Top => redraw_image(&mut world, &agent, &mut top_view_frame),
                    }
                }
                Message::MouseDown(x, y, button) => {
                    println!("The image was clicked at coordinates x={}, y={}", x, y);

                    /* if world.ellipses.len() == 0 {
                        let central_ellipse: Ellipse = Ellipse::new(
                            Coord::new((world.width as f32) / 2.0, (world.height as f32) / 2.0),
                            75.0,
                            50.0,
                            RGBAColor::new_rgb(255, 255, 0),
                        );

                        world.ellipses.push(central_ellipse);
                    } */

                    world.is_updated = true;
                }
                Message::MouseMove(current_x) => {
                    if mouse_x != -1 {
                        mouse_dx = current_x - mouse_x;
                    }
                    mouse_x = current_x;

                    if mouse_dx != 0 {
                        // println!("Mouse moved sideways: {}", mouse_dx);

                        /* agent.turn_sideways((mouse_dx as f32) / 3.0);
                        world.is_updated = true;
                        agent.is_updated = true; */
                    }
                    
                }
                Message::KeyPress(key_char) => {
                    match key_char {
                        'w' => {
                            // move forward
                            agent.move_forward(5.0);

                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        's' => {
                            // move backward
                            agent.move_forward(-5.0);
                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        'd' => {
                            // move right
                            agent.move_sideways(5.0);
                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        'a' => {
                            // move left
                            agent.move_sideways(-5.0);
                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        'e' => {
                            // rotate right
                            agent.turn_sideways(5.0);
                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        'q' => {
                            // rotate left
                            agent.turn_sideways(-5.0);
                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        'v' => {
                            match view_mode {
                                ViewMode::Top => view_mode = ViewMode::FirstPerson,
                                ViewMode::FirstPerson => view_mode = ViewMode::Top,
                            }
                            world.is_updated = true;
                            agent.is_updated = true;
                        }
                        _ => {}
                    }
                }
                _ => {
                    // println!("yet undefined event");
                }
            };
        }
    }

    application.run().unwrap();
}


fn redraw_image(world: &mut World, agent: &Agent, top_view_frame: &mut frame::Frame) {
    if world.is_updated {
        let rendered_scene: RGBACanvas = world.render_top_view(
            &agent.shape,
            agent.center,
            0.80,
            WIND_WIDTH,
            WIND_HEIGHT - MENU_HEIGHT,
        );

        let image = unsafe { RgbImage::from_data(
            &rendered_scene.data,
            rendered_scene.width,
            rendered_scene.height,
            ColorDepth::Rgba8,
        )
        .unwrap() };

        top_view_frame.set_image(Some(image));
        top_view_frame.redraw();

        world.is_updated = false;
    }
}

fn draw_fisrt_person_view(agent: &mut Agent, first_person_view_frame: &mut frame::Frame) {
    if agent.is_updated {
        let agent_line_view: Vec<RGBAColor> = agent.get_view(WIND_WIDTH);
        let mut agent_view: RGBACanvas = RGBACanvas::new(WIND_WIDTH, WIND_HEIGHT - MENU_HEIGHT);

        for j in 0..(WIND_HEIGHT - MENU_HEIGHT) {
            for i in 0..WIND_WIDTH {
                agent_view.put_pixel(i, j, agent_line_view[i as usize]);
            }
        }
    
        let image: RgbImage = unsafe { RgbImage::from_data(
            &agent_view.data,
            agent_view.width,
            agent_view.height,
            ColorDepth::Rgba8,
        )
        .unwrap() };
    
        first_person_view_frame.set_image(Some(image));
        first_person_view_frame.redraw();

        agent.is_updated = false;
    }
}

fn add_shapes_to_world(world: &mut World, shape_descriptions: Vec<ShapeDescription>) {
    let mut shapes: Vec<Shape> = Vec::new();

    for i in 0..shape_descriptions.len() {
        shapes.push(Shape::from_coord_list(
            String::from(&shape_descriptions[i].name),
            shape_descriptions[i].vertices.clone(),
            shape_descriptions[i].texture,
        ).unwrap());

        shapes[i].shift(shape_descriptions[i].anchor);
    }

    world.add_shapes_as_walls(&shapes);
}

fn get_init_data_from_file() -> Option<WorldSetup> {
    // Create a path to the desired file
    let path = Path::new("data/world-shapes.ron");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            return Some(ron::from_str(&s).unwrap());
        },
    }
}
