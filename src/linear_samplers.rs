use std::str::FromStr;

use device_query::{DeviceQuery, DeviceState, Keycode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::variable_holder::DataHolder;

pub struct SamplerData {
    pub t: f32,
    pub vars: DataHolder,
}

pub trait Sampler1D {
    fn sample(&mut self, data: &mut SamplerData) -> f32;
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
    Switch(Switch),
    KeyPress(KeyPress),
    DeltaTime(DeltaTime),
    VariableGet(VariableGet),
    VariableSet(VariableSet),
    Expressions(Expressions),
}

impl Sampler1D for Movesampler1D {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        match self {
            Movesampler1D::Constant(constant) => *constant,
            Movesampler1D::Time(time) => time.sample(data),
            Movesampler1D::MouseClick(mouse_click) => mouse_click.sample(data),
            Movesampler1D::Map(map) => map.sample(data),
            Movesampler1D::Add(add) => add.sample(data),
            Movesampler1D::Subtract(subtract) => subtract.sample(data),
            Movesampler1D::Multiply(multiply) => multiply.sample(data),
            Movesampler1D::Divide(divide) => divide.sample(data),
            Movesampler1D::Power(power) => power.sample(data),
            Movesampler1D::Modulo(modulo) => modulo.sample(data),
            // Movesampler1D::Trig(trig) => trig.sample(data),
            Movesampler1D::MouseClickCounter(mouse_click_counter) => {
                mouse_click_counter.sample(data)
            }
            Movesampler1D::CounterReset(counter_reset) => counter_reset.sample(data),
            Movesampler1D::Switch(switch) => switch.sample(data),
            Movesampler1D::KeyPress(key_press) => key_press.sample(data),
            Movesampler1D::DeltaTime(delta_time) => delta_time.sample(data),
            Movesampler1D::VariableGet(variable_get) => variable_get.sample(data),
            Movesampler1D::VariableSet(variable_set) => variable_set.sample(data),
            Movesampler1D::Expressions(expressions) => expressions.sample(data),
        }
    }
}

// #[derive(Debug, Serialize, Deserialize,JsonSchema)]
// pub struct Constant {
//     pub value: f32,
// }

// impl Sampler1D for Constant {
//     fn sample(&mut self, _data: &mut SamplerData) -> f32 {
//         self.value
//     }
// }

// impl From<Constant> for Movesampler1D {
//     fn from(c: Constant) -> Self {
//         Movesampler1D::Constant(c.value)
//     }
// }

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Time {
    pub speed: f32,
}

impl Sampler1D for Time {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.speed * data.t
    }
}

impl From<Time> for Movesampler1D {
    fn from(t: Time) -> Self {
        Movesampler1D::Time(t)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MouseClick {
    pub mouse_timer_decrease: Box<Movesampler1D>,
    pub mouse_button: usize,
    pub force_full_cycle: bool,
    #[serde(skip)]
    value: f32,
}

impl Sampler1D for MouseClick {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();
        if mouse.button_pressed[self.mouse_button] && self.value <= 0.0 {
            self.value = 1.0;
        }

        if self.value > 0.0 {
            self.value -= self.mouse_timer_decrease.sample(data);
        }

        self.value
    }
}

impl From<MouseClick> for Movesampler1D {
    fn from(m: MouseClick) -> Self {
        Movesampler1D::MouseClick(m)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Map {
    pub before_min: f32,
    pub before_max: f32,
    pub after_min: f32,
    pub after_max: f32,
    pub sampler: Box<Movesampler1D>,
}

impl Sampler1D for Map {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        let before = self.sampler.sample(data);
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Add {
    pub terms: Vec<Movesampler1D>,
}

impl Sampler1D for Add {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.terms
            .iter_mut()
            .fold(0.0, |acc, s| acc + s.sample(data))
    }
}

impl From<Add> for Movesampler1D {
    fn from(a: Add) -> Self {
        Movesampler1D::Add(a)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Subtract {
    pub pos: Box<Movesampler1D>,
    pub neg: Box<Movesampler1D>,
}

impl Sampler1D for Subtract {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.pos.sample(data) - self.neg.sample(data)
    }
}

impl From<Subtract> for Movesampler1D {
    fn from(s: Subtract) -> Self {
        Movesampler1D::Subtract(s)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Multiply {
    pub factors: Vec<Movesampler1D>,
}

impl Sampler1D for Multiply {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.factors
            .iter_mut()
            .fold(1.0, |acc, s| acc * s.sample(data))
    }
}

impl From<Multiply> for Movesampler1D {
    fn from(m: Multiply) -> Self {
        Movesampler1D::Multiply(m)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Divide {
    pub top: Box<Movesampler1D>,
    pub bottom: Box<Movesampler1D>,
}

impl Sampler1D for Divide {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.top.sample(data) / self.bottom.sample(data)
    }
}

impl From<Divide> for Movesampler1D {
    fn from(d: Divide) -> Self {
        Movesampler1D::Divide(d)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Power {
    pub base: Box<Movesampler1D>,
    pub exponent: Box<Movesampler1D>,
}

impl Sampler1D for Power {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.base.sample(data).powf(self.exponent.sample(data))
    }
}

impl From<Power> for Movesampler1D {
    fn from(p: Power) -> Self {
        Movesampler1D::Power(p)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Modulo {
    pub base: Box<Movesampler1D>,
    pub divisor: Box<Movesampler1D>,
}

impl Sampler1D for Modulo {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        self.base.sample(data) % self.divisor.sample(data)
    }
}

impl From<Modulo> for Movesampler1D {
    fn from(m: Modulo) -> Self {
        Movesampler1D::Modulo(m)
    }
}

// #[derive(Debug, Serialize, Deserialize,JsonSchema)]
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
//     fn sample(&mut self, data: &mut SamplerData) -> f32 {
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MouseClickCounter {
    pub mouse_click_counter_button: usize,
    #[serde(skip)]
    counter: u32,
}

impl Sampler1D for MouseClickCounter {
    fn sample(&mut self, _data: &mut SamplerData) -> f32 {
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CounterReset {
    pub counter: Box<Movesampler1D>,
    pub reset: Box<Movesampler1D>,
    #[serde(skip)]
    offset: f32,
}

impl Sampler1D for CounterReset {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        let reset = self.reset.sample(data);
        if reset >= 1.0 {
            self.offset = self.counter.sample(data);
        }
        self.counter.sample(data) - self.offset
    }
}

impl From<CounterReset> for Movesampler1D {
    fn from(c: CounterReset) -> Self {
        Movesampler1D::CounterReset(c)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Switch {
    pub enable: Box<Movesampler1D>,
    pub disable: Box<Movesampler1D>,
    #[serde(skip)]
    enabled: f32,
}

impl Sampler1D for Switch {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        let enable = self.enable.sample(data);
        let disable = self.disable.sample(data);
        if enable >= 1.0 {
            self.enabled = enable;
        }
        if disable >= 1.0 {
            self.enabled = 0.0;
        }
        self.enabled
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyPress {
    pub keys: Vec<String>,
    #[serde(skip)]
    fixed_keys: Vec<Keycode>,
}

impl Sampler1D for KeyPress {
    fn sample(&mut self, _data: &mut SamplerData) -> f32 {
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeltaTime {
    delta_time_multiplier: f32,
    #[serde(skip)]
    last_time: f32,
}

impl Sampler1D for DeltaTime {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        let delta_time = data.t - self.last_time;
        self.last_time = data.t;
        delta_time * self.delta_time_multiplier
    }
}

impl From<DeltaTime> for Movesampler1D {
    fn from(d: DeltaTime) -> Self {
        Movesampler1D::DeltaTime(d)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VariableGet {
    pub variable_name: String,
    #[serde(skip)]
    variable_id: Option<usize>,
}

impl Sampler1D for VariableGet {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        if self.variable_id.is_none() {
            self.variable_id = Some(data.vars.add_key(&self.variable_name));
        }

        data.vars.get(self.variable_id.unwrap())
    }
}

impl From<VariableGet> for Movesampler1D {
    fn from(v: VariableGet) -> Self {
        Movesampler1D::VariableGet(v)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VariableSet {
    pub set_variable_name: String,
    pub value: Box<Movesampler1D>,
    #[serde(skip)]
    variable_id: Option<usize>,
}

impl Sampler1D for VariableSet {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        if self.variable_id.is_none() {
            self.variable_id = Some(data.vars.add_key(&self.set_variable_name));
        }

        let value = self.value.sample(data);

        data.vars.set(self.variable_id.unwrap(), value);
        // println!("Set variable {} to {}", self.set_variable_name, value);

        value
    }
}

impl From<VariableSet> for Movesampler1D {
    fn from(v: VariableSet) -> Self {
        Movesampler1D::VariableSet(v)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Expressions {
    pub expressions: Vec<Movesampler1D>,
}

impl Sampler1D for Expressions {
    fn sample(&mut self, data: &mut SamplerData) -> f32 {
        // println!("Expressions: {:?}", self.expressions);
        self.expressions
            .iter_mut()
            .fold(0.0, |_acc, e| e.sample(data))
    }
}

impl From<Expressions> for Movesampler1D {
    fn from(e: Expressions) -> Self {
        Movesampler1D::Expressions(e)
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
                    mouse_timer_decrease: Box::new(Movesampler1D::Constant(0.01)),
                    mouse_button: 1,
                    force_full_cycle: false,
                    value: 0.0,
                }),
                Movesampler1D::Constant(50.0),
            ],
        });

        println!("{:?}", serde_jsonrc::to_string(&sampler).unwrap());
    }
}
