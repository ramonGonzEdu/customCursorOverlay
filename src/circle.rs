use std::f32::consts::PI;

use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Circle {
    pub color: Color,
    pub speed: f32,
    pub eccentricity: f32,
    pub angle_top: f32,
    pub angle_bottom: f32,
    pub radius: f32,
}

impl Circle {
    pub fn draw(&self, d: &mut RaylibDrawHandle, t: f32) {
        let pre_x = (self.speed * t).cos() * self.eccentricity;
        let pre_y = (self.speed * t).sin();

        let angle = (self.angle_top / self.angle_bottom) * 2.0_f32 * PI;

        let cos_v = angle.cos();
        let sin_v = angle.sin();

        let x = (pre_x * cos_v - pre_y * sin_v) * self.radius;
        let y = (pre_y * cos_v + pre_x * sin_v) * self.radius;

        d.draw_circle_gradient(
            50 + x as i32,
            50 + y as i32,
            4_f32,
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            self.color,
        );
    }
}
