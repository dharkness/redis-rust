use crate::commands::prelude::*;

struct Del {
    keys: Vec<String>,
}

impl Del {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Apply for Del {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        Ok(Response::Usize(
            self.keys
                .iter()
                .filter(|key| store.remove(key).is_some())
                .count(),
        ))
    }
}

pub struct DelParser {}

impl DelParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for DelParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Del::new(input.rest()?)))
    }
}
