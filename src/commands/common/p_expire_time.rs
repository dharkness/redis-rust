use crate::commands::prelude::*;

struct PExpireTime {
    key: String,
}

impl PExpireTime {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for PExpireTime {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if !store.contains_key(&self.key) {
            Ok(Response::I64(-2))
        } else if let Some(at) = store.expires(&self.key) {
            Ok(Response::I64(at.timestamp_millis()))
        } else {
            Ok(Response::I64(-1))
        }
    }
}

pub struct PExpireTimeParser {}

impl PExpireTimeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for PExpireTimeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(PExpireTime::new(input.next_string()?)))
    }
}
