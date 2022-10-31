use std::str::FromStr;

use super::{AttrParamsParser, AttributeValue};

pub struct AttributeFields {
    pub content: Option<Vec<u8>>,
}

impl AttributeFields {
    pub fn new(content: Option<Vec<u8>>) -> Self {
        Self { content }
    }

    fn get_param<'s>(&'s self, param_name: &str) -> Option<AttributeValue<'s>> {
        let content = self.content.as_ref()?;
        for (key, value) in AttrParamsParser::new(content.as_slice()) {
            if key == param_name {
                return Some(value);
            }
        }

        None
    }

    pub fn get_bool(&self, param_name: &str) -> Option<bool> {
        let value = self.get_param(param_name)?;

        match value.as_bool() {
            Ok(value) => Some(value),
            Err(err) => panic!("Can not read parameter {}. Err: {}", param_name, err),
        }
    }

    pub fn get_as_type<TFromStr: FromStr>(&self, param_name: &str) -> Option<TFromStr> {
        let attr_value = self.get_param(param_name)?;

        match attr_value.as_type() {
            Ok(value) => {
                return Some(value);
            }
            Err(err) => panic!("Can not read parameter {}. Err: {}", param_name, err),
        }
    }

    pub fn get_as_string(&self, param_name: &str) -> Option<String> {
        let attr_value = self.get_param(param_name)?;

        match attr_value.as_string() {
            Ok(value) => {
                return Some(value.to_string());
            }
            Err(err) => panic!("Can not read parameter {}. Err: {}", param_name, err),
        }
    }
}
