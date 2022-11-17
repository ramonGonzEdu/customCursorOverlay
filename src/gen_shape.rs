use raylib::prelude::RaylibDrawHandle;
use serde::{Deserialize, Serialize};

use crate::movement::{Movesampler2D, Sampler2D};

pub trait Drawable {
    fn draw(&mut self, d: &mut RaylibDrawHandle, t: f32, coords: (i32, i32));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    pub movement: Vec<Movesampler2D>,
    pub shape: ShapeRaw,
}

impl Drawable for Shape {
    fn draw(&mut self, d: &mut RaylibDrawHandle, t: f32, win: (i32, i32)) {
        let (x, y) = self.movement.iter_mut().fold((0.0, 0.0), |(x, y), m| {
            let (dx, dy) = m.sample(t);
            (x + dx, y + dy)
        });
        self.shape.draw(d, t, (x as i32 + win.0, y as i32 + win.1));
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShapeRaw {
    Circle(crate::circle::Circle),
}

impl Drawable for ShapeRaw {
    fn draw(&mut self, d: &mut RaylibDrawHandle, t: f32, pos: (i32, i32)) {
        match self {
            ShapeRaw::Circle(c) => c.draw(d, t, pos),
        }
    }
}
