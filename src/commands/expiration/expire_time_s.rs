use crate::commands::prelude::*;

struct ExpireTimeSecs {
    key: String,
}

impl ExpireTimeSecs {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for ExpireTimeSecs {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        if !store.contains_key(&self.key) {
            Ok(Response::I64(-2))
        } else if let Some(at) = store.expires(&self.key) {
            Ok(Response::I64(at.timestamp()))
        } else {
            Ok(Response::I64(-1))
        }
    }
}

pub struct ExpireTimeSecsParser {}

impl ExpireTimeSecsParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for ExpireTimeSecsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(ExpireTimeSecs::new(input.next_string()?)))
    }
}
