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

    pub fn as_string(&'s self) -> Result<&'s str, String> {
        match self {
            AttributeValue::Str(value) => Ok(value),
            _ => {
                let result = format!(
                    "Invalid parameter type. Expected string - found type: {}",
                    self.type_as_str()
                );

                Err(result)
            }
        }
    }

    pub fn as_type<TFromStr: FromStr>(&self) -> Result<TFromStr, String> {
        match self {
            AttributeValue::Number(value) => match FromStr::from_str(value) {
                Ok(result) => Ok(result),
                Err(_) => Err(format!("Can not parse value: {}", value)),
            },
            _ => {
                let result = format!(
                    "Invalid parameter type. Expected string - found type: {}",
                    self.type_as_str()
                );

                Err(result)
            }
        }
    }

    pub fn as_bool(&self) -> Result<bool, String> {
        match self {
            AttributeValue::Boolean(value) => Ok(*value),
            _ => {
                let result = format!(
                    "Invalid parameter type. Expected bool - found type: {}",
                    self.type_as_str()
                );

                Err(result)
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
