use std::f32::consts::PI;

use device_query::{DeviceQuery, DeviceState};
use serde::{Deserialize, Serialize};

pub trait Sampler2D {
    fn sample(&mut self, t: f32) -> (f32, f32);
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]

pub struct Orbit {
    pub speed: f32,
    pub eccentricity: f32,
    pub angle_top: f32,
    pub angle_bottom: f32,
    pub radius: f32,
}

impl Sampler2D for Orbit {
    fn sample(&mut self, t: f32) -> (f32, f32) {
        let pre_x = (self.speed * t).cos() * self.eccentricity;
        let pre_y = (self.speed * t).sin();

        let angle = (self.angle_top / self.angle_bottom) * 2.0_f32 * PI;

        let cos_v = angle.cos();
        let sin_v = angle.sin();

        let x = (pre_x * cos_v - pre_y * sin_v) * self.radius;
        let y = (pre_y * cos_v + pre_x * sin_v) * self.radius;

        return (x, y);
    }
}

impl From<Orbit> for Movesampler2D {
    fn from(o: Orbit) -> Self {
        Movesampler2D::Orbit(o)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Sampler2D for Offset {
    fn sample(&mut self, _t: f32) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl From<Offset> for Movesampler2D {
    fn from(o: Offset) -> Self {
        Movesampler2D::Offset(o)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mouse {
    pub scale_center_x: f32,
    pub scale_center_y: f32,
    pub scale: f32,
}

impl Sampler2D for Mouse {
    fn sample(&mut self, _t: f32) -> (f32, f32) {
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();

        let (x, y) = mouse.coords;
        let x = (x as f32 - self.scale_center_x) * self.scale;
        let y = (y as f32 - self.scale_center_y) * self.scale;
        (x, y)
    }
}
