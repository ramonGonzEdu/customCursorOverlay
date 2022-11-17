use raylib::prelude::*;
use serde::{Deserialize, Serialize};

use crate::gen_shape::{Drawable, ShapeRaw};

#[derive(Debug, Serialize, Deserialize)]
pub struct Circle {
    pub outer_color: Color,
    pub inner_color: Color,
    pub radius: f32,
}

impl Drawable for Circle {
    fn draw(&mut self, d: &mut RaylibDrawHandle, _t: f32, pos: (i32, i32)) {
        d.draw_circle_gradient(
            (pos.0) as i32,
            (pos.1) as i32,
            self.radius,
            self.inner_color,
            self.outer_color,
        );
    }
}

impl From<Circle> for ShapeRaw {
    fn from(c: Circle) -> Self {
        ShapeRaw::Circle(c)
    }
}
