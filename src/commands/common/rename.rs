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
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if store.rename(&self.key, &self.new_key) {
            client.write_ok(registry)
        } else {
            client.write_simple_error("key does not exist", registry)
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Rename::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}
