use super::prelude::*;

struct SetRemove {
    key: String,
    values: Vec<String>,
}

impl SetRemove {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for SetRemove {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match store.get_mut_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(ref mut members)) => {
                let mut removed = 0;
                for value in &self.values {
                    if members.remove(value) {
                        removed += 1;
                    }
                }
                if members.is_empty() {
                    store.remove(&self.key);
                }
                client.write_integer(removed, registry)
            }
            IfKindResult::NotSet => client.write_zero(registry),
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetRemoveParser {}

impl SetRemoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetRemoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetRemove::new(
            input.next_string()?,
            input.rest()?,
        )))
    }
}

enum When {
    Always,
    Exists,
    NotExists,
}
