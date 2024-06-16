use std::ops::Add;

use super::prelude::*;

struct Append {
    key: String,
    value: String,
}

impl Append {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Apply for Append {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let new_value = if let Some(value) = store.get(&self.key) {
            match value {
                Value::String(s) => s.clone().add(&self.value),
                _ => return client.write_simple_error(WRONG_TYPE, registry),
            }
        } else {
            self.value.clone()
        };

        let len = new_value.len();
        store.set(&self.key, Value::new_string(new_value));
        client.write_integer(len as i64, registry)
    }
}

pub struct AppendParser {}

impl AppendParser {
    pub const fn new() -> Self {
        Self {}
    }
}

impl TryParse for AppendParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Append::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}
