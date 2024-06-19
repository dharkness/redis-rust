use crate::commands::prelude::*;

struct Persist {
    key: String,
}

impl Persist {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Persist {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if store.contains_key(&self.key) && store.persist(&self.key) {
            Ok(Response::One)
        } else {
            Ok(Response::Zero)
        }
    }
}

pub struct PersistParser {}

impl PersistParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for PersistParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Persist::new(input.next_string()?)))
    }
}
