use std::collections::HashMap;

use super::AttributeFields;

pub struct Attributes {
    pub data: HashMap<String, AttributeFields>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, content: Option<Vec<u8>>) {
        self.data.insert(key, AttributeFields::new(content));
    }

    pub fn try_get(&self, name: &str) -> Option<&AttributeFields> {
        self.data.get(name)
    }

    pub fn get(&self, name: &str) -> &AttributeFields {
        match self.try_get(name) {
            Some(result) => result,
            None => panic!("Attribute {} is not found", name),
        }
    }
}
