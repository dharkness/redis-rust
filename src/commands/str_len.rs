use super::prelude::*;

struct StrLen {
    key: String,
}

impl StrLen {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for StrLen {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Some(value) = store.get(&self.key) {
            match value {
                Value::String(s) => client.write_integer(s.len() as i64, registry),
                _ => client.write_simple_error(WRONG_TYPE, registry),
            }
        } else {
            client.write_integer(0, registry)
        }
    }
}

pub struct StrLenParser {}

impl StrLenParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for StrLenParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(StrLen::new(input.next_string()?)))
    }
}
