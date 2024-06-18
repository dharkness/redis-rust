use crate::commands::prelude::*;

struct Echo {
    message: String,
}

impl Echo {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Apply for Echo {
    fn apply(&self, _store: &mut Store) -> Result<Response, Error> {
        Ok(Response::BulkString(self.message.clone()))
    }
}

pub struct EchoParser {}

impl EchoParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for EchoParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Echo::new(input.next_string()?)))
    }
}
