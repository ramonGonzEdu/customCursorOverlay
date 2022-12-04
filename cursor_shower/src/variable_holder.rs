use std::collections::HashMap;

pub struct DataHolder {
    pub key_data: HashMap<String, usize>,
    pub key_data_vec: Vec<f32>,
}

impl DataHolder {
    pub fn new() -> Self {
        Self {
            key_data: HashMap::new(),
            key_data_vec: Vec::new(),
        }
    }

    pub fn add_key(&mut self, key: &String) -> usize {
        if self.key_data.contains_key(key) {
            return self.key_data[key];
        }
        let index = self.key_data.len();
        self.key_data.insert(key.to_string(), index);
        self.key_data_vec.push(0.0);
        index
    }

    pub fn get_key(&self, key: &str) -> Option<&usize> {
        self.key_data.get(key)
    }

    pub fn set(&mut self, key: usize, value: f32) {
        self.key_data_vec[key] = value;
    }

    pub fn get(&self, key: usize) -> f32 {
        self.key_data_vec[key]
    }
}
