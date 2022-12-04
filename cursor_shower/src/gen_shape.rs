use raylib::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    linear_samplers::{Movesampler1D, Sampler1D, SamplerData},
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
    Rect(Rect),
}

impl Drawable for ShapeRaw {
    fn draw(&mut self, d: &mut RaylibDrawHandle, data: &mut SamplerData, pos: (i32, i32)) {
        match self {
            ShapeRaw::Circle(c) => c.draw(d, data, pos),
            ShapeRaw::NoDraw(_) => {}
            ShapeRaw::Rect(r) => r.draw(d, data, pos),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoDraw;

impl Drawable for NoDraw {
    fn draw(&mut self, _d: &mut RaylibDrawHandle, _data: &mut SamplerData, _pos: (i32, i32)) {}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Rect {
    pub width: Movesampler1D,
    pub height: Movesampler1D,
    pub color_tl: Color,
    pub color_tr: Color,
    pub color_bl: Color,
    pub color_br: Color,
}

impl Drawable for Rect {
    fn draw(&mut self, d: &mut RaylibDrawHandle, data: &mut SamplerData, pos: (i32, i32)) {
        d.draw_rectangle_gradient_ex(
            Rectangle {
                x: pos.0 as f32,
                y: pos.1 as f32,
                width: self.width.sample(data),
                height: self.height.sample(data),
            },
            self.color_tl,
            self.color_bl,
            self.color_br,
            self.color_tr,
        );
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
