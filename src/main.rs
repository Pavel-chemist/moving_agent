// Main things to do:
// make a first-person view for agent +
// make main view scalable

/* line textures...

procedural:
- edge textures -> change brightness / color depending on distance to the line ends, are symmetric around middle
  -- can be used to enchance or reduce corner visibility
  
- periodic -> anchored to the start of line, characterized by period and phase
  -- can be adjusted to seamlesly wrap around a polygon
  
  
  
Moving Agent features to add (with no particular order):
  
  1) pan and zoom for top-view
  2) move rendering (raster creation) of top-view and first-person view into separate threads
  3) add smooth transitions for agent movements
  4) add agent collisions with lines and polygons in world
  5) make prettier agent
  7) create dot display modes i.e. display only polygon vertices:
   -- 2d "wireframe",
   -- 2d "wireframe with transparent surfaces"
   -- 2d "occluded wireframe"
  8) add stereo modes (parallel-eye/cross-eye and anaglyph)
  9) world segmentation (this is to decrease computational load for collisions and renderings)
  11) investigate openGL api */

use common_structs::{Coord, Angle, RGBAColor};
use fltk::{
    app::{self, App, MouseButton},
    enums::{self, ColorDepth, FrameType, Event},
    image::RgbImage,
    prelude::*,
    *,
};

use rgba_canvas::RGBACanvas;
use world::World;

use ellipse::Ellipse;

mod common_structs;
mod line_seg;
mod vector_2d;
mod linear_texture;
mod rgba_canvas;
mod ellipse;
mod polygon;
mod shape;
mod agent;
mod world;

const WIND_LABEL: &str = "Moving Agent";

// big_window consts:
/* const WIND_WIDTH: i32 = 1820;
const WIND_HEIGHT: i32 = 1000;
const MAIN_IMAGE_WIDTH: i32 = 1560;
const MAIN_IMAGE_HEIGHT: i32 = 800; */

// small window consts
const WIND_WIDTH: i32 = 1000;
const WIND_HEIGHT: i32 = 720;
const MAIN_IMAGE_WIDTH: i32 = 780;
const MAIN_IMAGE_HEIGHT: i32 = 520;

const FIRST_PERSON_VIEW_HEIGHT: i32 = 128;

const MAIN_IMAGE_FRAME_THICKNESS: i32 = 4;
const MAIN_IMAGE_X_POS: i32 = 10;
const MAIN_IMAGE_Y_POS: i32 = 10;
const MENU_HEIGHT: i32 = 32;

#[derive(Clone)]
enum Message {
    Quit,
    MouseDown(i32, i32, MouseButton),
    MouseDrag(i32, i32),
    MouseMove(i32, i32),
    MouseReleased(i32, i32, MouseButton),
    Tick,
    KeyPress(char),
}

fn main() {
    let mut world: World = world::World::new(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    let application: App = app::App::default();

    let (s, r) = app::channel();

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

    let mut framing_frame_1 = frame::Frame::default()
        .with_pos(MAIN_IMAGE_X_POS, MAIN_IMAGE_Y_POS + MENU_HEIGHT)
        .with_size(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_FRAME_THICKNESS * 2,
            MAIN_IMAGE_HEIGHT + MAIN_IMAGE_FRAME_THICKNESS * 2,
        );
    framing_frame_1.set_frame(FrameType::EngravedBox);

    let mut top_view_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);


    let mut framing_frame_2 = frame::Frame::default()
        .with_pos(MAIN_IMAGE_X_POS, MAIN_IMAGE_Y_POS + MENU_HEIGHT + MAIN_IMAGE_FRAME_THICKNESS * 2 + MAIN_IMAGE_HEIGHT)
        .with_size(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_FRAME_THICKNESS * 2,
            FIRST_PERSON_VIEW_HEIGHT + MAIN_IMAGE_FRAME_THICKNESS * 2,
        );
    framing_frame_2.set_frame(FrameType::EngravedBox);

    let mut first_person_view_frame = frame::Frame::default()
    .with_pos(
        MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
        MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS * 3 + MENU_HEIGHT + MAIN_IMAGE_HEIGHT,
    )
    .with_size(MAIN_IMAGE_WIDTH, FIRST_PERSON_VIEW_HEIGHT);

    wind.end();
    wind.show();

    // intercept keyboard events on the window
    let key_interceptor_sender =s.clone();
    wind.handle(move |_, event| match event {
        Event::KeyDown => {
            match app::event_key().to_char() {
                Some(char) => {
                    key_interceptor_sender.send(Message::KeyPress(char));
                }
                None => {
                    // nothing
                }
            };
            // println!("Key '{:?}' was pressed.", app::event_key().to_char().unwrap());
            false
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
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                let button = app::event_mouse_button();
                top_view_frame_handle_sender.send(Message::MouseDown(x, y, button));
                true
            }
            Event::Drag => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                    top_view_frame_handle_sender.send(Message::MouseDrag(x, y));
                }
                true
            }
            Event::Released => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                let button = app::event_mouse_button();
                top_view_frame_handle_sender.send(Message::MouseReleased(x, y, button));
                true
            }
            Event::Move => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                    top_view_frame_handle_sender.send(Message::MouseMove(x, y));
                }
                true
            }
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

                    /* if world.shapes.len() > 0 {
                        for i in 0..(world.shapes.len()) {
                            world.shapes[i].rotate(Angle::new_deg(1.0 / ((i+3) as f32)));
                        }
                    }

                    world.is_updated = true;
                    world.agent.is_updated = true; */
                    redraw_image(&mut world, &mut top_view_frame);
                    draw_fisrt_person_view(&mut world, &mut first_person_view_frame);
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
                Message::KeyPress(key_char) => {
                    match key_char {
                        'w' => {
                            // move forward
                            world.agent.move_forward(5.0);
                            world.is_updated = true;
                            world.agent.is_updated = true;
                        }
                        's' => {
                            // move backward
                            world.agent.move_forward(-5.0);
                            world.is_updated = true;
                            world.agent.is_updated = true;
                        }
                        'd' => {
                            // move right
                            world.agent.move_sideways(5.0);
                            world.is_updated = true;
                            world.agent.is_updated = true;
                        }
                        'a' => {
                            // move left
                            world.agent.move_sideways(-5.0);
                            world.is_updated = true;
                            world.agent.is_updated = true;
                        }
                        'e' => {
                            // rotate right
                            world.agent.turn_sideways(5.0);
                            world.is_updated = true;
                            world.agent.is_updated = true;
                        }
                        'q' => {
                            // rotate left
                            world.agent.turn_sideways(-5.0);
                            world.is_updated = true;
                            world.agent.is_updated = true;
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


fn redraw_image(world: &mut World, top_view_frame: &mut frame::Frame) {
    if world.is_updated {
        let mut rendered_scene: RGBACanvas = world.static_background.clone();

        /* for i in 0..world.lines.len() {
            world.lines[i].draw(&mut rendered_scene);
        }

        for i in 0..world.polygons.len() {
            world.polygons[i].draw(&mut rendered_scene);
        }

        for i in 0..world.ellipses.len() {
            world.ellipses[i].draw_ellipse_raster(&mut rendered_scene, false, 20.0);
        } */

        for i in 0..world.shapes.len() {
            world.shapes[i].draw(&mut rendered_scene);
        }

        world.agent.draw(&mut rendered_scene);

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

fn draw_fisrt_person_view(world: &mut World, first_person_view_frame: &mut frame::Frame) {
    if world.agent.is_updated {
        let agent_line_view: Vec<RGBAColor> = world.agent.get_view(MAIN_IMAGE_WIDTH, &world);
        let mut agent_view: RGBACanvas = RGBACanvas::new(MAIN_IMAGE_WIDTH, FIRST_PERSON_VIEW_HEIGHT);
    
        for j in 0..FIRST_PERSON_VIEW_HEIGHT {
            for i in 0..MAIN_IMAGE_WIDTH {
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

        world.agent.is_updated = false;
    }
}
