use device_query::{DeviceQuery, DeviceState};
use serde::{Deserialize, Serialize};

pub trait Sampler1D {
    fn sample(&mut self, t: f32) -> f32;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "untagged")]
pub enum Movesampler1D {
    Constant(f32),
    Time(Time),
    MouseClick(MouseClick),
    Map(Map),
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Power(Power),
    Modulo(Modulo),
    Trig(Trig),
}

impl Sampler1D for Movesampler1D {
    fn sample(&mut self, t: f32) -> f32 {
        match self {
            Movesampler1D::Constant(constant) => *constant,
            Movesampler1D::Time(time) => time.sample(t),
            Movesampler1D::MouseClick(mouse_click) => mouse_click.sample(t),
            Movesampler1D::Map(map) => map.sample(t),
            Movesampler1D::Add(add) => add.sample(t),
            Movesampler1D::Subtract(subtract) => subtract.sample(t),
            Movesampler1D::Multiply(multiply) => multiply.sample(t),
            Movesampler1D::Divide(divide) => divide.sample(t),
            Movesampler1D::Power(power) => power.sample(t),
            Movesampler1D::Modulo(modulo) => modulo.sample(t),
            Movesampler1D::Trig(trig) => trig.sample(t),
        }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Constant {
//     pub value: f32,
// }

// impl Sampler1D for Constant {
//     fn sample(&mut self, _t: f32) -> f32 {
//         self.value
//     }
// }

// impl From<Constant> for Movesampler1D {
//     fn from(c: Constant) -> Self {
//         Movesampler1D::Constant(c.value)
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub speed: f32,
}

impl Sampler1D for Time {
    fn sample(&mut self, t: f32) -> f32 {
        self.speed * t
    }
}

impl From<Time> for Movesampler1D {
    fn from(t: Time) -> Self {
        Movesampler1D::Time(t)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MouseClick {
    pub timer_tick: f32,
    value: f32,
}

impl Sampler1D for MouseClick {
    fn sample(&mut self, t: f32) -> f32 {
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();
        if mouse.button_pressed[1] {
            self.value = 1.0;
        }

        if self.value > 0.0 {
            self.value -= self.timer_tick;
        }

        self.value
    }
}

impl From<MouseClick> for Movesampler1D {
    fn from(m: MouseClick) -> Self {
        Movesampler1D::MouseClick(m)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub before_min: f32,
    pub before_max: f32,
    pub after_min: f32,
    pub after_max: f32,
    pub sampler: Box<Movesampler1D>,
}

impl Sampler1D for Map {
    fn sample(&mut self, t: f32) -> f32 {
        let before = self.sampler.sample(t);
        let before_range = self.before_max - self.before_min;
        let after_range = self.after_max - self.after_min;
        let before_normalized = (before - self.before_min) / before_range;
        let after = (before_normalized * after_range) + self.after_min;
        after
    }
}

impl From<Map> for Movesampler1D {
    fn from(m: Map) -> Self {
        Movesampler1D::Map(m)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Add {
    pub samplers: Vec<Movesampler1D>,
}

impl Sampler1D for Add {
    fn sample(&mut self, t: f32) -> f32 {
        self.samplers
            .iter_mut()
            .fold(0.0, |acc, s| acc + s.sample(t))
    }
}

impl From<Add> for Movesampler1D {
    fn from(a: Add) -> Self {
        Movesampler1D::Add(a)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtract {
    pub pos: Box<Movesampler1D>,
    pub neg: Box<Movesampler1D>,
}

impl Sampler1D for Subtract {
    fn sample(&mut self, t: f32) -> f32 {
        self.pos.sample(t) - self.neg.sample(t)
    }
}

impl From<Subtract> for Movesampler1D {
    fn from(s: Subtract) -> Self {
        Movesampler1D::Subtract(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Multiply {
    pub samplers: Vec<Movesampler1D>,
}

impl Sampler1D for Multiply {
    fn sample(&mut self, t: f32) -> f32 {
        self.samplers
            .iter_mut()
            .fold(1.0, |acc, s| acc * s.sample(t))
    }
}

impl From<Multiply> for Movesampler1D {
    fn from(m: Multiply) -> Self {
        Movesampler1D::Multiply(m)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Divide {
    pub top: Box<Movesampler1D>,
    pub bottom: Box<Movesampler1D>,
}

impl Sampler1D for Divide {
    fn sample(&mut self, t: f32) -> f32 {
        self.top.sample(t) / self.bottom.sample(t)
    }
}

impl From<Divide> for Movesampler1D {
    fn from(d: Divide) -> Self {
        Movesampler1D::Divide(d)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Power {
    pub base: Box<Movesampler1D>,
    pub exponent: Box<Movesampler1D>,
}

impl Sampler1D for Power {
    fn sample(&mut self, t: f32) -> f32 {
        self.base.sample(t).powf(self.exponent.sample(t))
    }
}

impl From<Power> for Movesampler1D {
    fn from(p: Power) -> Self {
        Movesampler1D::Power(p)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modulo {
    pub number: Box<Movesampler1D>,
    pub divisor: Box<Movesampler1D>,
}

impl Sampler1D for Modulo {
    fn sample(&mut self, t: f32) -> f32 {
        self.number.sample(t) % self.divisor.sample(t)
    }
}

impl From<Modulo> for Movesampler1D {
    fn from(m: Modulo) -> Self {
        Movesampler1D::Modulo(m)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Trig {
    Sin(Box<Movesampler1D>),
    Cos(Box<Movesampler1D>),
    Tan(Box<Movesampler1D>),
    Asin(Box<Movesampler1D>),
    Acos(Box<Movesampler1D>),
    Atan(Box<Movesampler1D>),
}

impl Sampler1D for Trig {
    fn sample(&mut self, t: f32) -> f32 {
        match self {
            Trig::Sin(s) => s.sample(t).sin(),
            Trig::Cos(s) => s.sample(t).cos(),
            Trig::Tan(s) => s.sample(t).tan(),
            Trig::Asin(s) => s.sample(t).asin(),
            Trig::Acos(s) => s.sample(t).acos(),
            Trig::Atan(s) => s.sample(t).atan(),
        }
    }
}

impl From<Trig> for Movesampler1D {
    fn from(t: Trig) -> Self {
        Movesampler1D::Trig(t)
    }
}
