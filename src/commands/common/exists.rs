use crate::commands::prelude::*;

struct Exists {
    keys: Vec<String>,
}

impl Exists {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for Exists {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        Ok(Response::Usize(
            self.keys
                .iter()
                .filter(|key| store.contains_key(key))
                .count(),
        ))
    }
}

pub struct ExistsParser {}

impl ExistsParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for ExistsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Exists::new(input.rest()?)))
    }
}
