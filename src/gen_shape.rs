use raylib::prelude::RaylibDrawHandle;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    linear_samplers::{Movesampler1D, SamplerData},
    movement::{Movesampler2D, Sampler2D},
};

pub trait Drawable {
    fn draw(&mut self, d: &mut RaylibDrawHandle, t: &mut SamplerData, coords: (i32, i32));
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Shape {
    pub enabled: Movesampler1D,
    pub movement: Vec<Movesampler2D>,
    pub shape: ShapeRaw,
}

impl Drawable for Shape {
    fn draw(&mut self, d: &mut RaylibDrawHandle, data: &mut SamplerData, win: (i32, i32)) {
        let (x, y) = self.movement.iter_mut().fold((0.0, 0.0), |(x, y), m| {
            let (dx, dy) = m.sample(data);
            (x + dx, y + dy)
        });
        self.shape
            .draw(d, data, (x as i32 + win.0, y as i32 + win.1));
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ShapeRaw {
    Circle(crate::circle::Circle),
    NoDraw(NoDraw),
}

impl Drawable for ShapeRaw {
    fn draw(&mut self, d: &mut RaylibDrawHandle, data: &mut SamplerData, pos: (i32, i32)) {
        match self {
            ShapeRaw::Circle(c) => c.draw(d, data, pos),
            ShapeRaw::NoDraw(_) => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoDraw;

impl Drawable for NoDraw {
    fn draw(&mut self, _d: &mut RaylibDrawHandle, _data: &mut SamplerData, _pos: (i32, i32)) {}
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
