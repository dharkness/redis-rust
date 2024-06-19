use crate::commands::prelude::*;

struct ExpireTimeMillis {
    key: String,
}

impl ExpireTimeMillis {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for ExpireTimeMillis {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        if !store.contains_key(&self.key) {
            Ok(Response::I64(-2))
        } else if let Some(at) = store.expires(&self.key) {
            Ok(Response::I64(at.timestamp_millis()))
        } else {
            Ok(Response::I64(-1))
        }
    }
}

pub struct ExpireTimeMillisParser {}

impl ExpireTimeMillisParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for ExpireTimeMillisParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(ExpireTimeMillis::new(input.next_string()?)))
    }
}
