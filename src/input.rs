use crate::parser::parse_integer;

pub struct Input<'a> {
    tokens: Vec<&'a str>,
    index: usize,
}

impl<'a> Input<'a> {
    pub fn new(tokens: Vec<&'a str>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.tokens.len() - self.index
    }

    pub fn has_next(&self) -> bool {
        self.index < self.tokens.len()
    }

    pub fn next(&mut self) -> Result<&'a str, String> {
        if !self.has_next() {
            Err("no more tokens".to_string())
        } else {
            self.index += 1;
            Ok(self.tokens[self.index - 1])
        }
    }

    pub fn next_string(&mut self) -> Result<String, String> {
        Ok(self.next()?.to_string())
    }

    pub fn next_token(&mut self) -> Result<String, String> {
        Ok(self.next()?.to_uppercase())
    }

    pub fn next_int(&mut self) -> Result<i64, String> {
        parse_integer(self.next()?.as_bytes())
    }

    pub fn rest(&mut self) -> Result<Vec<String>, String> {
        let rest = self.tokens[self.index..]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        self.index = self.tokens.len();
        Ok(rest)
    }
}
