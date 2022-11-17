pub mod circle;
pub mod gen_shape;
pub mod linear_samplers;
pub mod movement;

use device_query::{DeviceQuery, DeviceState, MouseState};
use raylib::prelude::*;

use crate::gen_shape::{Drawable, Shape};

fn main() {
    let s = 100;

    let (mut rl, thread) = raylib::init()
        .size(s, s)
        .title("RCC")
        .transparent()
        .undecorated()
        .build();

    // let wx = (1920 - s) >> 1;
    // let wy = (1080 - s) >> 1;
    // rl.set_window_position(wx, wy);
    let mut t = 0.0_f32;

    let (width, height) = unsafe {
        let monitor = raylib::ffi::GetCurrentMonitor();
        (
            raylib::ffi::GetMonitorWidth(monitor),
            raylib::ffi::GetMonitorHeight(monitor),
        )
    };

    let sw = width - 20;
    let sh = height - 20;
    rl.set_window_position(10, 10);
    rl.set_window_size(sw, sh);

    let mut cursor: Vec<Shape> =
        serde_json::from_reader(std::fs::File::open("cursor.json").unwrap()).unwrap();

    println!("serialized = {}", serde_json::to_string(&cursor).unwrap());
    let device_state = DeviceState::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // rl.get_mouse_position()
        // println!("{:?}", MouseCursor::pos());

        let mouse: MouseState = device_state.get_mouse();
        println!("Current Mouse Coordinates: {:?}", mouse.coords);
        println!("Current Mouse Clicks: {:?}", mouse.button_pressed);

        t += d.get_frame_time();

        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        for shape in &mut cursor {
            shape.draw(&mut d, t, (sw >> 2, sh >> 2));
            // shape.draw(&mut d, t, (pos.0 - 10, pos.1 - 10));
        }
    }
}
