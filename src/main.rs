use common_structs::{Coord, RGBACanvas, Angle};
use fltk::{
    app::{self, App, MouseButton},
    enums::{self, ColorDepth, FrameType, Event},
    image::RgbImage,
    prelude::*,
    *,
};

use world::World;

use crate::{common_structs::RGBAColor, ellipse::Ellipse};

mod common_structs;
mod world;
mod line;
mod ellipse;
mod polygon;

const WIND_LABEL: &str = "Moving Agent";

// big_window consts:
/* const WIND_WIDTH: i32 = 1820;
const WIND_HEIGHT: i32 = 1000;
const MAIN_IMAGE_WIDTH: i32 = 1560;
const MAIN_IMAGE_HEIGHT: i32 = 940; */

// small window consts
const WIND_WIDTH: i32 = 800;
const WIND_HEIGHT: i32 = 600;
const MAIN_IMAGE_WIDTH: i32 = 780;
const MAIN_IMAGE_HEIGHT: i32 = 520;

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

    let mut framing_frame = frame::Frame::default()
        .with_pos(MAIN_IMAGE_X_POS, MAIN_IMAGE_Y_POS + MENU_HEIGHT)
        .with_size(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_FRAME_THICKNESS * 2,
            MAIN_IMAGE_HEIGHT + MAIN_IMAGE_FRAME_THICKNESS * 2,
        );
    framing_frame.set_frame(FrameType::EngravedBox);

    let mut top_view_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

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

                    if world.shapes.len() > 3 {
                        world.shapes[2].rotate(Angle::new_f(0.01));

                        world.is_updated = true;
                    }

                    redraw_image(&mut world, &mut top_view_frame);
                }
                Message::MouseDown(x, y, button) => {
                    println!("The image was clicked at coordinates x={}, y={}", x, y);

                    if world.ellipses.len() == 0 {
                        let central_ellipse: Ellipse = Ellipse::new(
                            Coord::new((world.width as f64) / 2.0, (world.height as f64) / 2.0),
                            75.0,
                            50.0,
                            RGBAColor::new_rgb(255, 255, 0),
                        );

                        world.ellipses.push(central_ellipse);
                    }

                    world.is_updated = true;
                }
                Message::KeyPress(key_char) => {
                    match key_char {
                        'w' => {
                            // move forward
                            world.ellipses[0].center.move_y(-5.0);
                            world.is_updated = true;
                        }
                        's' => {
                            // move backward
                            world.ellipses[0].center.move_y(5.0);
                            world.is_updated = true;
                        }
                        'd' => {
                            // move right
                            world.ellipses[0].center.move_x(5.0);
                            world.is_updated = true;
                        }
                        'a' => {
                            // move left
                            world.ellipses[0].center.move_x(-5.0);
                            world.is_updated = true;
                        }
                        'e' => {
                            // rotate right
                            world.ellipses[0].angle.turn(0.05);
                            world.is_updated = true;
                        }
                        'q' => {
                            // rotate left
                            world.ellipses[0].angle.turn(-0.05);
                            world.is_updated = true;
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


fn redraw_image(world_state: &mut World, image_frame: &mut frame::Frame) {
    if world_state.is_updated {

        let image_data: RGBACanvas = world_state.get_rendered_view();

        let image = RgbImage::new(
            &image_data.data,
            image_data.width,
            image_data.height,
            ColorDepth::Rgba8,
        )
        .unwrap();

        image_frame.set_image(Some(image));
        image_frame.redraw();

        world_state.is_updated = false;
    }
}
