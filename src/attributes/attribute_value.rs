use std::str::FromStr;

pub enum AttributeValue<'s> {
    Str(&'s str),
    Boolean(bool),
    Number(&'s str),
    None,
}

impl<'s> AttributeValue<'s> {
    pub fn parse(src: &'s [u8]) -> Self {
        if src[0] == b'"' {
            return Self::Str(std::str::from_utf8(&src[1..src.len() - 1]).unwrap());
        }

        if src == b"true" {
            return Self::Boolean(true);
        }

        if src == b"false" {
            return Self::Boolean(false);
        }

        Self::Number(std::str::from_utf8(src).unwrap())
    }

    pub fn type_as_str(&self) -> &str {
        match self {
            Self::Str(_) => "string",
            Self::Boolean(_) => "boolean",
            Self::Number(_) => "number",
            Self::None => "none",
        }
    }

    pub fn unwrap_as_string(&self) -> &str {
        match self {
            AttributeValue::Str(value) => value,
            _ => {
                panic!(
                    "Invalid parameter type. Expected string - found type: {}",
                    self.type_as_str()
                )
            }
        }
    }

    pub fn unwrap_as_type<TFromStr: FromStr>(&self) -> usize {
        match self {
            AttributeValue::Number(value) => FromStr::from_str(value).unwrap(),
            _ => {
                panic!(
                    "Invalid parameter type. Expected string - found type: {}",
                    self.type_as_str()
                )
            }
        }
    }

    pub fn unwrap_as_bool(&self) -> bool {
        match self {
            AttributeValue::Boolean(value) => *value,
            _ => {
                panic!(
                    "Invalid parameter type. Expected bool - found type: {}",
                    self.type_as_str()
                )
            }
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            AttributeValue::None => true,
            _ => false,
        }
    }
}
