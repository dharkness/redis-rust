use crate::commands::prelude::*;

struct Rename {
    key: String,
    new_key: String,
}

impl Rename {
    pub fn new(key: String, new_key: String) -> Self {
        Self { key, new_key }
    }
}

impl Apply for Rename {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        if store.rename(&self.key, &self.new_key) {
            Ok(Response::Ok)
        } else {
            Err(Error::KeyNotFound)
        }
    }
}

pub struct RenameParser {}

impl RenameParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RenameParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Rename::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}
