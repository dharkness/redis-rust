use super::parser::{parse_i64, parse_u64};

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

    pub fn next_strings(&mut self, count: usize) -> Result<Vec<String>, String> {
        if self.len() < count {
            return Err("Invalid arguments specified for command".to_string());
        }
        let strings = self.tokens[self.index..self.index + count]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        self.index += count;
        Ok(strings)
    }

    pub fn next_token(&mut self) -> Result<String, String> {
        Ok(self.next()?.to_uppercase())
    }

    pub fn next_i64(&mut self) -> Result<i64, String> {
        parse_i64(self.next()?.as_bytes())
    }

    pub fn next_u64(&mut self) -> Result<u64, String> {
        parse_u64(self.next()?.as_bytes())
    }

    pub fn next_u64_min(&mut self, min: u64) -> Result<u64, String> {
        let value = parse_u64(self.next()?.as_bytes())?;
        if value >= min {
            Ok(value)
        } else {
            Err("Invalid arguments specified for command".to_string())
        }
    }

    pub fn next_count(&mut self) -> Result<usize, String> {
        let count = self.next_u64()?;
        if count > 0 {
            Ok(count as usize)
        } else {
            Err("Invalid arguments specified for command".to_string())
        }
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
