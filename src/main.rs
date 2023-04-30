use common_structs::{Coord, RGBCanvas};
use fltk::{
    app::{self, App, MouseButton},
    enums::{self, Color, ColorDepth, Event, FrameType},
    image::RgbImage,
    prelude::*,
    *,
};

use state::State;

mod common_structs;
mod state;
mod circle;

const WIND_LABEL: &str = "Floating Objects";
// const WIND_WIDTH: i32 = 1820;
const WIND_WIDTH: i32 = 800;
// const WIND_HEIGHT: i32 = 1000;
const WIND_HEIGHT: i32 = 600;
// const MAIN_IMAGE_WIDTH: i32 = 940;
// const MAIN_IMAGE_WIDTH: i32 = 1560;
const MAIN_IMAGE_WIDTH: i32 = 512;
// const MAIN_IMAGE_HEIGHT: i32 = 940;
const MAIN_IMAGE_HEIGHT: i32 = 512;
const MAIN_IMAGE_FRAME_THICKNESS: i32 = 4;
const MAIN_IMAGE_X_POS: i32 = 10;
const MAIN_IMAGE_Y_POS: i32 = 10;
const MENU_HEIGHT: i32 = 32;

#[derive(Clone)]
enum Message {
    Quit,
    AddCircleButEv,
    RemoveCircleButEv,
    WBev,
    BBev,
    GBev,
    LGBev,
    MouseDown(i32, i32, MouseButton),
    MouseDrag(i32, i32),
    MouseMove(i32, i32),
    MouseReleased(i32, i32, MouseButton),
    Tick,
}

enum Colour {
    Black,
    Grey,
    LightGrey,
    White,
}

fn main() {
    let mut world_state: State = state::State::new(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    let application: App = app::App::default();
    let mut is_added_circle: bool = false;
    let mut new_circle_coord: Coord = Coord::new(0.0, 0.0);
    let mut cursor_coord: Coord = Coord::new(0.0, 0.0);

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

    // the image_frame is used to show generated image
    let mut image_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    // the ghost_frame acts as mouse events interceptor
    let mut ghost_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);
    

    let mut b_add_circle = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS,
        200,
        40,
        "Click to add circle",
    );
    b_add_circle.emit(s.clone(), Message::AddCircleButEv);

    let mut b_remove_circle = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 50,
        200,
        40,
        "Click to remove circles",
    );
    b_remove_circle.emit(s.clone(), Message::RemoveCircleButEv);

    let _change_color_title_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
            MENU_HEIGHT + MAIN_IMAGE_Y_POS + 100,
        )
        .with_size(200, 40)
        .with_label("Change background color:");

    let mut b_white = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_white.set_color(Color::White);
    b_white.emit(s.clone(), Message::WBev);

    let mut b_grey = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 1,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_grey.set_color(Color::rgb_color(127, 127, 127));
    b_grey.emit(s.clone(), Message::GBev);

    let mut b_light_grey = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 2,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_light_grey.set_color(Color::FrameDefault);
    b_light_grey.emit(s.clone(), Message::LGBev);

    let mut b_black = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 3,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_black.set_color(Color::Black);
    b_black.emit(s.clone(), Message::BBev);

    let mut _total_momentum_display_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
            MENU_HEIGHT + MAIN_IMAGE_Y_POS + 200,
        )
        .with_size(200, 40)
        .with_label("");

    wind.end();
    wind.show();

    let callback_sender = s.clone();
    
    let callback = move |handle| {
        callback_sender.send(Message::Tick);
        
        app::repeat_timeout3(0.016667, handle);
    };
    

    let ghost_frame_handle_sender = s.clone();
    ghost_frame.handle(move |_, event: Event| {
        match event {
            Event::Push => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                let button = app::event_mouse_button();
                ghost_frame_handle_sender.send(Message::MouseDown(x, y, button));
                true
            }
            Event::Drag => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                    ghost_frame_handle_sender.send(Message::MouseDrag(x, y));
                }
                true
            }
            Event::Released => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                let button = app::event_mouse_button();
                ghost_frame_handle_sender.send(Message::MouseReleased(x, y, button));
                true
            }
            Event::Move => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                    ghost_frame_handle_sender.send(Message::MouseMove(x, y));
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
                Message::AddCircleButEv => {
                    println!("Adding circle...");

                    world_state.add_random_circle_at_coords(MAIN_IMAGE_WIDTH / 2, MAIN_IMAGE_HEIGHT / 2, 0.0, 0.0);
                }
                Message::RemoveCircleButEv => {
                    println!("Removing circle...");

                    world_state.circles = Vec::new();
                }
                Message::WBev => {
                    println!("Change background to White.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::White)
                    );
                }
                Message::LGBev => {
                    println!("Change background to Light Grey.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::LightGrey)
                    );
                }
                Message::GBev => {
                    println!("Change background to Grey.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::Grey)
                    );
                }
                Message::BBev => {
                    println!("Change background to Black.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::Black)
                    );
                }
                Message::Tick => {
                    redraw_image(
                        &mut world_state,
                        &mut image_frame,
                        is_added_circle,
                        &new_circle_coord,
                        &cursor_coord,
                    );
                    
                    _total_momentum_display_frame.set_label(&(String::from("Total momentum is:\n") + &format!("{:>10.3}", world_state.get_total_momentum())));
                }
                Message::MouseDown(x, y, button) => {
                    println!("The image was clicked at coordinates x={}, y={}", x, y);

                    if button == MouseButton::Right {
                        world_state.select_circle(x, y);
                    }
                    
                    if button == MouseButton::Left {
                        if !is_added_circle {
                            world_state.add_random_circle_at_coords(x, y, 0.0, 0.0);
                            new_circle_coord.set_i(x, y);
                            cursor_coord.set_i(x, y);
                            is_added_circle = true;
                        } else {
                            // let new_coord: Coord = Coord::new_i(x, y);
                            cursor_coord.set_i(x, y);
                            let index: usize = world_state.circles.len()-1;

                            world_state.circles[index].x_vel = (cursor_coord.x - new_circle_coord.x) / 100.0;
                            world_state.circles[index].y_vel = (cursor_coord.y - new_circle_coord.y) / 100.0;
                            is_added_circle = false;
                        }
                    }
                }
                Message::MouseDrag(x, y) => {
                    let circle_index: usize = world_state.selected_circle_index;

                    if world_state.has_selected_circle {
                        world_state.circles[circle_index].accelerate_to_position(x as f64, y as f64);
                    }
                }
                Message::MouseMove(x, y) => {
                    // println!("There was Move event at coordinates x={}, y={}", x, y);
                    if is_added_circle {
                        // draw a line from circle center to current mouse cursor
                        cursor_coord.set_i(x, y);
                    }
                } 
                _ => {
                    println!("yet undefined event");
                }
            };
        }
    }

    application.run().unwrap();
}


fn redraw_image(world_state: &mut State, image_frame: &mut frame::Frame, is_line: bool, start: &Coord, end: &Coord) {
    world_state.progress_one_step();
    let mut image_data = world_state.get_rendered_view();
    if is_line {
        draw_line(&mut image_data, start, end);
    }

    let image = RgbImage::new(
        &image_data.data,
        image_data.width as i32,
        image_data.height as i32,
        ColorDepth::Rgb8,
    )
    .unwrap();
    image_frame.set_image(Some(image));
    image_frame.redraw();
}

fn generate_image_background(width: i32, height: i32, colour: Colour) -> Vec<u8> {
    let num_pix: usize = (width * height) as usize;

    let data_array: Vec<u8>;

    match colour {
        Colour::Black => data_array = vec![0; num_pix * 3],
        Colour::Grey => data_array = vec![127; num_pix * 3],
        Colour::LightGrey => data_array = vec![191; num_pix * 3],
        Colour::White => data_array = vec![255; num_pix * 3],
    }

    return data_array;
}

fn draw_line(image_data: &mut RGBCanvas, start: &Coord, end: &Coord) {
    let width: i32 = image_data.width as i32;
    
    let x_s: i32 = start.x as i32;
    let y_s: i32 = start.y as i32;
    
    let delta_x: f64 = end.x - start.x;
    let delta_y: f64 = end.y - start.y;
    let t: f64;

    let step_x: f64;
    let step_y: f64;

    let mut x: i32;
    let mut y: i32;

    if delta_x > 0.0 && delta_y > 0.0 {
        t = delta_x + delta_y;
    } else if delta_x > 0.0 && delta_y < 0.0 {
        t = delta_x - delta_y;
    } else if delta_x < 0.0 && delta_y > 0.0 {
        t = - delta_x + delta_y;
    } else if delta_x < 0.0 && delta_y < 0.0 {
        t = - delta_x - delta_y;
    } else {
        t = 1.0;
    }

    step_x = delta_x / t;
    step_y = delta_y / t;

    for t in 0..(t as usize + 1) {
        x = x_s + (t as f64 * step_x) as i32;
        y = y_s + (t as f64 * step_y) as i32;

        image_data.data[(y * width * 3 + x * 3 + 0) as usize] = 255;
        image_data.data[(y * width * 3 + x * 3 + 1) as usize] = 255;
        image_data.data[(y * width * 3 + x * 3 + 2) as usize] = 255;
    }

}
