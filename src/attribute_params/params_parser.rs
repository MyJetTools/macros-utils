use crate::{AttrParamsParser, ParamValue};

pub struct Position {
    pub from: usize,
    pub to: usize,
}

impl Position {
    pub fn get_str<'s>(&self, src: &'s str) -> &'s str {
        &src[self.from..self.to]
    }
}

pub enum ParamsType {
    Single(Position),
    Multiple(Vec<(Position, Position)>),
}

pub struct AttributeParams {
    src: String,
    params: Option<ParamsType>,
}

impl AttributeParams {
    pub fn new(src: String) -> Self {
        let mut result = Self { src, params: None };

        if let Some(single_pos) = is_single_value(result.src.as_str()) {
            result.params = Some(ParamsType::Single(single_pos));
        } else {
            result.params = Some(ParamsType::Multiple(
                AttrParamsParser::new(result.src.as_bytes()).collect(),
            ));
        }

        result
    }

    pub fn get_single_param<'s>(&'s self) -> Option<ParamValue<'s>> {
        let result = self.params.as_ref()?;

        match result {
            ParamsType::Single(value) => Some(ParamValue {
                value: self.src[value.from..value.to].as_bytes(),
            }),
            _ => None,
        }
    }

    pub fn get_named_param<'s>(&'s self, param_name: &str) -> Option<ParamValue<'s>> {
        let result = self.params.as_ref()?;

        match result {
            ParamsType::Multiple(key_values) => {
                for (key, value) in key_values {
                    let key = key.get_str(&self.src);

                    if key == param_name {
                        return Some(ParamValue {
                            value: value.get_str(&self.src).as_bytes(),
                        });
                    }
                }

                None
            }
            _ => None,
        }
    }

    pub fn get_from_single_or_named<'s>(&'s self, param_name: &str) -> Option<ParamValue<'s>> {
        if let Some(result) = self.get_single_param() {
            return Some(result);
        }

        self.get_named_param(param_name)
    }
}

fn is_single_value(src: &str) -> Option<Position> {
    if src.starts_with('"') {
        return Some(Position {
            from: 1,
            to: src.len() - 1,
        });
    }

    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_simple_params() {
        let params = r#"a: "1", b: "2""#;

        let result = super::AttributeParams::new(params.to_string());

        assert_eq!("1", result.get_named_param("a").unwrap().get_value_as_str());
        assert_eq!("2", result.get_named_param("b").unwrap().get_value_as_str());

        assert_eq!(
            "2",
            result
                .get_from_single_or_named("b")
                .unwrap()
                .get_value_as_str()
        );
    }

    #[test]
    fn test_params_with_number_and_bool() {
        let params = r#"a: 1, b: true"#;

        let result = super::AttributeParams::new(params.to_string());

        assert_eq!(1, result.get_named_param("a").unwrap().get_value());
        assert_eq!(true, result.get_named_param("b").unwrap().get_value());
    }
}
