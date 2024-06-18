use crate::commands::prelude::*;

struct SetMultiple {
    key_value_pairs: Vec<String>,
}

impl SetMultiple {
    pub fn new(key_value_pairs: Vec<String>) -> Self {
        Self { key_value_pairs }
    }
}

impl Apply for SetMultiple {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if self.key_value_pairs.len() % 2 != 0 {
            Err(Error::Syntax)
        } else {
            for i in (0..self.key_value_pairs.len()).step_by(2) {
                store.set(
                    &self.key_value_pairs[i],
                    Value::from(self.key_value_pairs[i + 1].clone()),
                );
            }
            Ok(Response::Ok)
        }
    }
}

pub struct SetMultipleParser {}

impl SetMultipleParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetMultipleParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetMultiple::new(input.rest()?)))
    }
}
