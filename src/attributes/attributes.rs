use std::collections::HashMap;

use super::AttributeFields;

pub struct Attributes {
    data: HashMap<String, AttributeFields>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, content: Vec<u8>) {
        self.data.insert(key, AttributeFields::new(content));
    }
}
