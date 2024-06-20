use crate::network::Error;

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

    pub fn next(&mut self) -> Result<&'a str, Error> {
        if !self.has_next() {
            Err(Error::Syntax)
        } else {
            self.index += 1;
            Ok(self.tokens[self.index - 1])
        }
    }

    pub fn next_string(&mut self) -> Result<String, Error> {
        Ok(self.next()?.to_string())
    }

    pub fn next_strings(
        &mut self,
        command: &str,
        arg: &str,
        count: usize,
    ) -> Result<Vec<String>, Error> {
        if self.len() < count {
            return Err(Error::MissingArgument(command.to_string(), arg.to_string()));
        }
        let strings = self.tokens[self.index..self.index + count]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        self.index += count;
        Ok(strings)
    }

    pub fn next_token(&mut self) -> Result<String, Error> {
        Ok(self.next()?.to_uppercase())
    }

    pub fn next_i64(&mut self) -> Result<i64, Error> {
        parse_i64(self.next()?.as_bytes())
    }

    pub fn next_u64(&mut self) -> Result<u64, Error> {
        parse_u64(self.next()?.as_bytes())
    }

    pub fn next_usize(&mut self) -> Result<usize, Error> {
        parse_u64(self.next()?.as_bytes()).map(|n| n as usize)
    }

    pub fn next_u64_min(&mut self, min: u64) -> Result<u64, Error> {
        let value = parse_u64(self.next()?.as_bytes())?;
        if value >= min {
            Ok(value)
        } else {
            Err(Error::Integer)
        }
    }

    pub fn rest(&mut self) -> Result<Vec<String>, Error> {
        let rest = self.tokens[self.index..]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        self.index = self.tokens.len();
        Ok(rest)
    }
}
