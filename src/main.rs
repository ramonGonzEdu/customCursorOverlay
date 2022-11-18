// #![windows_subsystem = "windows"]
pub mod circle;
pub mod gen_shape;
pub mod hash_string;
pub mod linear_samplers;
pub mod movement;
pub mod serde_keycode_serialize;
pub mod variable_holder;

use raylib::prelude::*;

use crate::{
    gen_shape::{Drawable, Shape},
    linear_samplers::{Sampler1D, SamplerData},
    variable_holder::DataHolder,
};

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

    let (width, height) = unsafe {
        let monitor = raylib::ffi::GetCurrentMonitor();
        (
            raylib::ffi::GetMonitorWidth(monitor) >> 1,
            raylib::ffi::GetMonitorHeight(monitor) >> 1,
        )
    };

    let sw = width - 20;
    let sh = height - 20;
    rl.set_window_position(10, 10);
    rl.set_window_size(sw, sh);

    // rl.set_target_fps(5);

    let mut cursor: Vec<Shape> =
        serde_jsonrc::from_reader(std::fs::File::open("cursor.jsonc").unwrap()).unwrap();

    println!("serialized = {}", serde_jsonrc::to_string(&cursor).unwrap());
    // let device_state = DeviceState::new();

    let mut data = SamplerData {
        t: 0.0,
        vars: DataHolder::new(),
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // rl.get_mouse_position()
        // println!("{:?}", MouseCursor::pos());

        // let mouse: MouseState = device_state.get_mouse();
        // println!("Current Mouse Coordinates: {:?}", mouse.coords);
        // println!("Current Mouse Clicks: {:?}", mouse.button_pressed);

        data.t += d.get_frame_time();

        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        for shape in &mut cursor {
            if shape.enabled.sample(&mut data) >= 1.0 {
                shape.draw(&mut d, &mut data, (sw >> 1, sh >> 1));
            }
            // shape.draw(&mut d, t, (pos.0 - 10, pos.1 - 10));
        }
    }
}
