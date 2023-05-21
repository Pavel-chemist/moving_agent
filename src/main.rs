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

use linear_texture::{LinearTexture, TransType, TextType};
use rgba_canvas::RGBACanvas;
use shape::Shape;
use world::World;

use ellipse::Ellipse;

mod common_structs;
mod vector_2d;
mod linear_texture;
mod rgba_canvas;
mod ellipse;
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
    MouseMove(i32),
    MouseReleased(i32, i32, MouseButton),
    Tick,
    KeyPress(char),
}

fn main() {
    let mut world: World = world::World::new(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);
    add_walls_to_world(&mut world);

    let mut agent: Agent = Agent::new(
        // Coord::new_i(width / 2, height / 2),
        Coord::new_i(200, 310),
        Angle::new_deg(0.0),
        Angle::new_deg(120.0),
        1000.0,
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
            /* Event::Move => {
                let mut x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let mut y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
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
                    // 

                    /* if world.shapes.len() > 0 {
                        for i in 0..(world.shapes.len()) {
                            world.shapes[i].rotate(Angle::new_deg(1.0 / ((i+3) as f32)));
                        }
                    }

                    world.is_updated = true;
                    world.agent.is_updated = true; */
                    redraw_image(&mut world, &agent, &mut top_view_frame);
                    draw_fisrt_person_view(&mut agent, &world, &mut first_person_view_frame);
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
        let mut rendered_scene: RGBACanvas = world.static_background.clone();

        agent.draw(&mut rendered_scene);

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

fn draw_fisrt_person_view(agent: &mut Agent, world: &World, first_person_view_frame: &mut frame::Frame) {
    if agent.is_updated {
        let agent_line_view: Vec<RGBAColor> = agent.get_view(MAIN_IMAGE_WIDTH/* , &world.walls */);
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

        agent.is_updated = false;
    }
}

fn add_walls_to_world(world: &mut World) {
    let mut shapes: Vec<Shape> = Vec::new();

    shapes.push(Shape::new_box(
        String::from("Box shape"),
        100.0,
        200.0,
        LinearTexture::new(
            RGBAColor::new_p(Palette::Grass),
            RGBAColor::new_p(Palette::White),
            10.0,
            TransType::Lin,
            RGBAColor::new_p(Palette::DarkGreen),
            70.0,
            0.0,
            TextType::Step,
            0.2,
        ),
    ).unwrap());
    shapes[0].shift(Coord::new(500.0, 300.0));
    shapes[0].rotate(Angle::new_deg(11.0));
    
    shapes.push(Shape::new_regular_polygon(
        String::from("Pentagon shape"),
        100.0,
        5,
        LinearTexture::new(
            RGBAColor::new_p(Palette::Red),
            RGBAColor::new_p(Palette::Yellow),
            10.0,
            TransType::Quad,
            RGBAColor::new_p(Palette::DarkRed),
            20.0,
            0.0,
            TextType::Step,
            0.3333,
        ),
    ).unwrap());
    shapes[1].shift(Coord::new(150.0, 200.0));
    shapes[1].rotate(Angle::new_deg(-11.0));


    shapes.push(Shape::new_box(
        String::from("Top wall"),
        800.0,
        20.0,
        LinearTexture::new(
            RGBAColor::new_p(Palette::Yellow),
            RGBAColor::new_p(Palette::White),
            10.0,
            TransType::Lin,
            RGBAColor::new_p(Palette::Orange),
            50.0,
            0.0,
            TextType::Step,
            0.2,
        ),
    ).unwrap());
    shapes[2].shift(Coord::new(400.0, 10.0));

    shapes.push(Shape::new_box(
        String::from("Bottom wall"),
        800.0,
        20.0,
        LinearTexture::new(
            RGBAColor::new_p(Palette::Cyan),
            RGBAColor::new_p(Palette::White),
            10.0,
            TransType::Lin,
            RGBAColor::new_p(Palette::Blue),
            50.0,
            0.0,
            TextType::Step,
            0.2,
        ),
    ).unwrap());
    shapes[3].shift(Coord::new(400.0, 510.0));

    shapes.push(Shape::new_box(
        String::from("Left wall"),
        20.0,
        520.0,
        LinearTexture::new(
            RGBAColor::new_p(Palette::Orange),
            RGBAColor::new_p(Palette::White),
            10.0,
            TransType::Lin,
            RGBAColor::new_p(Palette::Red),
            50.0,
            0.0,
            TextType::Step,
            0.2,
        ),
    ).unwrap());
    shapes[4].shift(Coord::new(10.0, 260.0));

    shapes.push(Shape::new_box(
        String::from("Left wall"),
        20.0,
        520.0,
        LinearTexture::new(
            RGBAColor::new_p(Palette::Green),
            RGBAColor::new_p(Palette::White),
            10.0,
            TransType::Lin,
            RGBAColor::new_p(Palette::DarkGreen),
            50.0,
            0.0,
            TextType::Step,
            0.2,
        ),
    ).unwrap());
    shapes[5].shift(Coord::new(770.0, 260.0));

    shapes.push(Shape::new_box(
        String::from("Middle wall"),
        20.0,
        400.0,
        LinearTexture::new(
            RGBAColor::new_p(Palette::LightGrey),
            RGBAColor::new_p(Palette::White),
            10.0,
            TransType::Lin,
            RGBAColor::new_p(Palette::DarkGrey),
            30.0,
            0.0,
            TextType::Step,
            0.3333,
        ),
    ).unwrap());
    shapes[6].shift(Coord::new(350.0, 320.0));

    world.add_shapes_as_walls(&shapes);
}
