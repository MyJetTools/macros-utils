use super::AttributeValue;

pub enum TokenResult {
    Found,
    Eol,
    InvalidToken(u8),
    None,
}

impl TokenResult {
    pub fn is_end_of_line(&self) -> bool {
        match self {
            TokenResult::Eol => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            TokenResult::None => true,
            _ => false,
        }
    }
}

pub struct AttrParamsParser<'s> {
    line: &'s [u8],
    found_eol: bool,

    key_start: usize,
    key_end: usize,
    value_start: usize,
    value_end: usize,
}

impl<'s> AttrParamsParser<'s> {
    pub fn new(line: &'s [u8]) -> Self {
        Self {
            line,
            key_start: 0,
            key_end: 0,
            value_start: 0,
            value_end: 0,
            found_eol: false,
        }
    }

    pub fn find_key_start(&mut self) -> Option<()> {
        for i in self.key_start..self.line.len() {
            if self.line[i] == b')' {
                self.found_eol = true;
                return None;
            }

            if self.line[i] == '(' as u8 {
                continue;
            }

            if self.line[i] == '*' as u8 {
                continue;
            }

            if is_params_separator(self.line[i]) {
                continue;
            }

            if self.line[i] == 32 {
                continue;
            }

            self.key_start = i;
            return Some(());
        }

        None
    }

    fn fine_key_end(&mut self) -> Option<()> {
        for i in self.key_start..self.line.len() {
            let b = self.line[i];

            if is_params_separator(b) {
                self.key_end = i;
                return Some(());
            }

            if b == '=' as u8 || b == 32 || b == ')' as u8 {
                self.key_end = i;
                return Some(());
            }
        }

        None
    }

    fn fine_value_start(&mut self) -> TokenResult {
        for i in self.key_end..self.line.len() {
            let b = self.line[i];

            if is_params_separator(b) {
                self.value_end = i;
                return TokenResult::None;
            }

            if b == '=' as u8 {
                continue;
            }

            if b == ')' as u8 {
                self.found_eol = true;
                return TokenResult::Eol;
            }

            if b > 32 {
                self.value_start = i;
                return TokenResult::Found;
            }
        }

        self.found_eol = true;
        TokenResult::Eol
    }

    fn fine_value_end(&mut self) -> Option<()> {
        if self.line[self.value_start] == b'"' || self.line[self.value_start] == b'\'' {
            let start_quote = self.line[self.value_start];

            for i in self.value_start + 1..self.line.len() {
                let b = self.line[i];

                if b == start_quote as u8 {
                    self.value_end = i + 1;
                    return Some(());
                }
            }
        } else {
            for i in self.value_start + 1..self.line.len() {
                let b = self.line[i];

                if b == ' ' as u8 || is_params_separator(b) || b == ')' as u8 {
                    self.value_end = i;
                    return Some(());
                }
            }
        }

        None
    }
}

impl<'s> Iterator for AttrParamsParser<'s> {
    type Item = (&'s str, AttributeValue<'s>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.found_eol {
            return None;
        }

        self.key_start = self.value_end + 1;

        self.find_key_start()?;
        self.fine_key_end();

        let key = &self.line[self.key_start..self.key_end];
        let key = std::str::from_utf8(key).unwrap();

        let result = self.fine_value_start();

        if result.is_end_of_line() {
            return Some((key, AttributeValue::None));
        }

        if result.is_none() {
            return Some((key, AttributeValue::None));
        }

        self.fine_value_end()?;

        let value = &self.line[self.value_start..self.value_end];

        Some((key, AttributeValue::parse(value)))
    }
}

fn is_params_separator(b: u8) -> bool {
    b == ',' as u8 || b == ';' as u8
}

#[cfg(test)]
mod tests {
    use crate::attributes::AttributeValue;

    use super::AttrParamsParser;

    #[test]
    fn test_basic_parse() {
        let src = r#"(name = "operationsPerPage" ; description = "Messages per page"; default)"#;

        let result = AttrParamsParser::new(src.as_bytes()).collect::<Vec<(&str, AttributeValue)>>();

        assert_eq!(result[0].0, "name");
        assert_eq!(result[0].1.as_string().unwrap(), "operationsPerPage");

        assert_eq!(result[1].0, "description");
        assert_eq!(result[1].1.as_string().unwrap(), "Messages per page");

        assert_eq!(result[2].0, "default");
        assert!(result[2].1.is_none());
    }

    #[test]
    fn test_parse_with_empty_param() {
        let src = r#"(name = "operationsPerPage" ; description = "Messages per page"; default;)"#;

        let result = AttrParamsParser::new(src.as_bytes()).collect::<Vec<(&str, AttributeValue)>>();

        assert_eq!(result[0].0, "name");
        assert_eq!(result[0].1.as_string().unwrap(), "operationsPerPage");

        assert_eq!(result[1].0, "description");
        assert_eq!(result[1].1.as_string().unwrap(), "Messages per page");

        assert_eq!(result[2].0, "default");
        assert!(result[2].1.is_none());
    }

    #[test]
    fn test_parse_with_two_empty_params_at_the_end() {
        let src =
            r#"(name = "operationsPerPage" ; description = "Messages per page"; default;default2)"#;

        let result = AttrParamsParser::new(src.as_bytes()).collect::<Vec<(&str, AttributeValue)>>();

        assert_eq!(result[0].0, "name");
        assert_eq!(result[0].1.as_string().unwrap(), "operationsPerPage");

        assert_eq!(result[1].0, "description");
        assert_eq!(result[1].1.as_string().unwrap(), "Messages per page");

        assert_eq!(result[2].0, "default");
        assert!(result[2].1.is_none());

        assert_eq!(result[3].0, "default2");
        assert!(result[3].1.is_none());
    }

    #[test]
    fn test_parse_with_boolean_true() {
        let src = r#"(name = "operationsPerPage" , amount = 15; do_it=true, undo=false)"#;

        let result = AttrParamsParser::new(src.as_bytes()).collect::<Vec<(&str, AttributeValue)>>();

        assert_eq!(result[0].0, "name");
        assert_eq!(result[0].1.as_string().unwrap(), "operationsPerPage");

        assert_eq!(result[1].0, "amount");
        assert_eq!(result[1].1.as_type::<usize>().unwrap(), 15);

        assert_eq!(result[2].0, "do_it");
        assert!(result[2].1.as_bool().unwrap());

        assert_eq!(result[3].0, "undo");
        assert!(!result[3].1.as_bool().unwrap());
    }
}
