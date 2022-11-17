use std::str::FromStr;

use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};

pub trait Sampler1D {
    fn sample(&mut self, t: f32) -> f32;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
    // Trig(Trig),
    MouseClickCounter(MouseClickCounter),
    CounterReset(CounterReset),
    KeyPress(KeyPress),
    DeltaTime(DeltaTime),
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
            // Movesampler1D::Trig(trig) => trig.sample(t),
            Movesampler1D::MouseClickCounter(mouse_click_counter) => mouse_click_counter.sample(t),
            Movesampler1D::CounterReset(counter_reset) => counter_reset.sample(t),
            Movesampler1D::KeyPress(key_press) => key_press.sample(t),
            Movesampler1D::DeltaTime(delta_time) => delta_time.sample(t),
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
    pub mouse_timer_decrease: f32,
    #[serde(skip)]
    value: f32,
}

impl Sampler1D for MouseClick {
    fn sample(&mut self, _t: f32) -> f32 {
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();
        if mouse.button_pressed[1] {
            self.value = 1.0;
        }

        if self.value > 0.0 {
            self.value -= self.mouse_timer_decrease;
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
    pub terms: Vec<Movesampler1D>,
}

impl Sampler1D for Add {
    fn sample(&mut self, t: f32) -> f32 {
        self.terms.iter_mut().fold(0.0, |acc, s| acc + s.sample(t))
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
    pub factors: Vec<Movesampler1D>,
}

impl Sampler1D for Multiply {
    fn sample(&mut self, t: f32) -> f32 {
        self.factors
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
    pub base: Box<Movesampler1D>,
    pub divisor: Box<Movesampler1D>,
}

impl Sampler1D for Modulo {
    fn sample(&mut self, t: f32) -> f32 {
        self.base.sample(t) % self.divisor.sample(t)
    }
}

impl From<Modulo> for Movesampler1D {
    fn from(m: Modulo) -> Self {
        Movesampler1D::Modulo(m)
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type")]
// pub enum Trig {
//     Sin(Box<Movesampler1D>),
//     Cos(Box<Movesampler1D>),
//     Tan(Box<Movesampler1D>),
//     Asin(Box<Movesampler1D>),
//     Acos(Box<Movesampler1D>),
//     Atan(Box<Movesampler1D>),
// }

// impl Sampler1D for Trig {
//     fn sample(&mut self, t: f32) -> f32 {
//         match self {
//             Trig::Sin(s) => s.sample(t).sin(),
//             Trig::Cos(s) => s.sample(t).cos(),
//             Trig::Tan(s) => s.sample(t).tan(),
//             Trig::Asin(s) => s.sample(t).asin(),
//             Trig::Acos(s) => s.sample(t).acos(),
//             Trig::Atan(s) => s.sample(t).atan(),
//         }
//     }
// }

// impl From<Trig> for Movesampler1D {
//     fn from(t: Trig) -> Self {
//         Movesampler1D::Trig(t)
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct MouseClickCounter {
    pub mouse_click_counter_button: usize,
    #[serde(skip)]
    counter: u32,
}

impl Sampler1D for MouseClickCounter {
    fn sample(&mut self, _t: f32) -> f32 {
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();

        if mouse.button_pressed[self.mouse_click_counter_button] {
            self.counter += 1;
        }
        self.counter as f32
    }
}

impl From<MouseClickCounter> for Movesampler1D {
    fn from(m: MouseClickCounter) -> Self {
        Movesampler1D::MouseClickCounter(m)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterReset {
    pub counter: Box<Movesampler1D>,
    pub reset: Box<Movesampler1D>,
    #[serde(skip)]
    offset: f32,
}

impl Sampler1D for CounterReset {
    fn sample(&mut self, t: f32) -> f32 {
        let reset = self.reset.sample(t);
        if reset >= 1.0 {
            self.offset = self.counter.sample(t);
        }
        self.counter.sample(t) - self.offset
    }
}

impl From<CounterReset> for Movesampler1D {
    fn from(c: CounterReset) -> Self {
        Movesampler1D::CounterReset(c)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPress {
    pub keys: Vec<String>,
    #[serde(skip)]
    fixed_keys: Vec<Keycode>,
}

impl Sampler1D for KeyPress {
    fn sample(&mut self, _t: f32) -> f32 {
        if self.fixed_keys.len() == 0 {
            self.fixed_keys = self
                .keys
                .iter()
                .map(|k| Keycode::from_str(k).expect("Invalid key"))
                .collect();
        }

        let device_state = DeviceState::new();
        let keyboard = device_state.get_keys();

        let mut pressed = 0.0;
        for k in self.fixed_keys.iter() {
            if keyboard.contains(k) {
                pressed += 1.0;
            }
        }

        pressed
    }
}

impl From<KeyPress> for Movesampler1D {
    fn from(k: KeyPress) -> Self {
        Movesampler1D::KeyPress(k)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeltaTime {
    delta_time_multiplier: f32,
    #[serde(skip)]
    last_time: f32,
}

impl Sampler1D for DeltaTime {
    fn sample(&mut self, t: f32) -> f32 {
        let delta_time = t - self.last_time;
        self.last_time = t;
        delta_time * self.delta_time_multiplier
    }
}

impl From<DeltaTime> for Movesampler1D {
    fn from(d: DeltaTime) -> Self {
        Movesampler1D::DeltaTime(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_mouse() {
        // create a sampler that multiplies the mouse click falloff by 50
        let sampler = Movesampler1D::Multiply(Multiply {
            factors: vec![
                Movesampler1D::MouseClick(MouseClick {
                    mouse_timer_decrease: 0.01,
                    value: 0.0,
                }),
                Movesampler1D::Constant(50.0),
            ],
        });

        println!("{:?}", serde_jsonrc::to_string(&sampler).unwrap());
    }
}
