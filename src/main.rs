pub mod circle;

use std::f32::consts::PI;

use raylib::prelude::*;

fn main() {
    let s = 100;

    let (mut rl, thread) = raylib::init()
        .size(s, s)
        .title("Hello, World")
        .transparent()
        .undecorated()
        .build();

    let wx = (1920 - s) >> 1;
    let wy = (1080 - s) >> 1;
    let mut t = 0.0_f32;

    let highlight = Color {
        r: 255,
        g: 50,
        b: 255,
        a: 255,
    };

    rl.set_window_position(wx, wy);

    let mut circles = vec![
        circle::Circle {
            angle_top: 0.0,
            angle_bottom: 5.0,
            eccentricity: 0.5_f32,
            speed: 6.0_f32,
            radius: 10.0_f32,
            color: highlight,
        },
        circle::Circle {
            angle_top: 1.0,
            angle_bottom: 5.0,
            eccentricity: 0.5_f32,
            speed: 6.0_f32,
            radius: 10.0_f32,
            color: highlight,
        },
        circle::Circle {
            angle_top: 2.0,
            angle_bottom: 5.0,
            eccentricity: 0.5_f32,
            speed: 6.0_f32,
            radius: 10.0_f32,
            color: highlight,
        },
        circle::Circle {
            angle_top: 3.0,
            angle_bottom: 5.0,
            eccentricity: 0.5_f32,
            speed: 6.0_f32,
            radius: 10.0_f32,
            color: highlight,
        },
        circle::Circle {
            angle_top: 4.0,
            angle_bottom: 5.0,
            eccentricity: 0.5_f32,
            speed: 6.0_f32,
            radius: 10.0_f32,
            color: highlight,
        },
        circle::Circle {
            angle_top: 0.0,
            angle_bottom: 5.0,
            eccentricity: 1.0_f32,
            speed: 6.0_f32,
            radius: 0.0_f32,
            color: Color {
                r: 128,
                g: 64,
                b: 128,
                a: 255,
            },
        },
    ];

    println!("serialized = {}", serde_json::to_string(&circles).unwrap());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        t += d.get_frame_time();

        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        for circle in circles.iter_mut() {
            circle.draw(&mut d, t);
        }
    }
}
