use super::prelude::*;

struct GetDel {
    key: String,
}

impl GetDel {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for GetDel {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Some(value) = store.get_and_remove(&self.key) {
            match value {
                Value::String(s) => client.write_bulk_string(&s, registry),
                _ => client.write_simple_error(WRONG_TYPE, registry),
            }
        } else {
            client.write_null(registry)
        }
    }
}

pub struct GetDelParser {}

impl GetDelParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetDelParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(GetDel::new(input.next_string()?)))
    }
}
