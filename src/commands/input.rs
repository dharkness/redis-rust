pub struct Input {
    tokens: Vec<String>,
}

impl Input {
    pub fn new(tokens: Vec<String>) -> Self {
        Self{ tokens }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn has_next(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub fn next(&mut self) -> Result<String, String> {
        if self.tokens.is_empty() {
            Err("missing token".to_string())
        } else {
            Ok(self.tokens.remove(0))
        }
    }

    pub fn next_token(&mut self) -> Result<String, String> {
        Ok(self.next()?.to_uppercase())
    }

    pub fn next_int(&mut self) -> Result<u64, String> {
        self.next()?.parse::<u64>().map_err(|e| e.to_string())
    }

    pub fn rest(&mut self) -> Result<Vec<String>, String> {
        Ok(self.tokens.split_off(0))
    }
}
