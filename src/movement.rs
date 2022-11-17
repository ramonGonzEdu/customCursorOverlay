use std::f32::consts::PI;

use device_query::{DeviceQuery, DeviceState};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::linear_samplers::{Movesampler1D, Sampler1D};

pub trait Sampler2D {
    fn sample(&mut self, t: f32) -> (f32, f32);
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Movesampler2D {
    Orbit(Orbit),
    Offset(Offset),
    Mouse(Mouse),
}

impl Sampler2D for Movesampler2D {
    fn sample(&mut self, t: f32) -> (f32, f32) {
        match self {
            Movesampler2D::Orbit(o) => o.sample(t),
            Movesampler2D::Offset(o) => o.sample(t),
            Movesampler2D::Mouse(m) => m.sample(t),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct Orbit {
    pub speed: Movesampler1D,
    pub eccentricity: Movesampler1D,
    pub angle_top: Movesampler1D,
    pub angle_bottom: Movesampler1D,
    pub radius: Movesampler1D,
}

impl Sampler2D for Orbit {
    fn sample(&mut self, t: f32) -> (f32, f32) {
        let pre_x = (self.speed.sample(t) * t).cos() * self.eccentricity.sample(t);
        let pre_y = (self.speed.sample(t) * t).sin();

        let angle = (self.angle_top.sample(t) / self.angle_bottom.sample(t)) * 2.0_f32 * PI;

        let cos_v = angle.cos();
        let sin_v = angle.sin();

        let x = (pre_x * cos_v - pre_y * sin_v) * self.radius.sample(t);
        let y = (pre_y * cos_v + pre_x * sin_v) * self.radius.sample(t);

        return (x, y);
    }
}

impl From<Orbit> for Movesampler2D {
    fn from(o: Orbit) -> Self {
        Movesampler2D::Orbit(o)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Offset {
    pub x: Movesampler1D,
    pub y: Movesampler1D,
}

impl Sampler2D for Offset {
    fn sample(&mut self, t: f32) -> (f32, f32) {
        (self.x.sample(t), self.y.sample(t))
    }
}

impl From<Offset> for Movesampler2D {
    fn from(o: Offset) -> Self {
        Movesampler2D::Offset(o)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Mouse {
    pub scale_center_x: Movesampler1D,
    pub scale_center_y: Movesampler1D,
    pub scale: Movesampler1D,
}

impl Sampler2D for Mouse {
    fn sample(&mut self, t: f32) -> (f32, f32) {
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();

        let (x, y) = mouse.coords;
        let x = (x as f32 - self.scale_center_x.sample(t)) * self.scale.sample(t);
        let y = (y as f32 - self.scale_center_y.sample(t)) * self.scale.sample(t);
        (x, y)
    }
}
