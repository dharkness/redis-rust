use super::prelude::*;

struct Get {
    key: String,
}

impl Get {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Get {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Some(value) = store.get(&self.key) {
            client.write_bulk_string(value, registry)
        } else {
            client.write_null(registry)
        }
    }
}

pub struct GetParser {}

impl GetParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Get::new(input.next_string()?)))
    }
}
