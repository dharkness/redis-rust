use crate::commands::prelude::*;

struct ExpireTime {
    key: String,
}

impl ExpireTime {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for ExpireTime {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if !store.contains_key(&self.key) {
            Ok(Response::I64(-2))
        } else if let Some(at) = store.expires(&self.key) {
            Ok(Response::I64(at.timestamp()))
        } else {
            Ok(Response::I64(-1))
        }
    }
}

pub struct ExpireTimeParser {}

impl ExpireTimeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for ExpireTimeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(ExpireTime::new(input.next_string()?)))
    }
}
