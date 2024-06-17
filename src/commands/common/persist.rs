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
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if store.contains_key(&self.key) && store.persist(&self.key) {
            client.write_integer(1, registry)
        } else {
            client.write_integer(0, registry)
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Persist::new(input.next_string()?)))
    }
}
