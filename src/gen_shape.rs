use raylib::prelude::RaylibDrawHandle;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    linear_samplers::Movesampler1D,
    movement::{Movesampler2D, Sampler2D},
};

pub trait Drawable {
    fn draw(&mut self, d: &mut RaylibDrawHandle, t: f32, coords: (i32, i32));
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Shape {
    pub enabled: Movesampler1D,
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_schema() {
        let schema = schemars::schema_for!(Vec<Shape>);
        println!("{}", serde_jsonrc::to_string_pretty(&schema).unwrap());
        // Write to schema.json
        std::fs::write(
            "schema.json",
            serde_jsonrc::to_string_pretty(&schema).unwrap(),
        )
        .unwrap();
    }
}
