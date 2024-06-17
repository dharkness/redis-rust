use crate::commands::prelude::*;

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
        match store.get_mut_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(ref mut s)) => {
                s.push_str(&self.value);
                client.write_integer(s.len() as i64, registry)
            }
            IfKindResult::NotSet => {
                store.set(&self.key, Value::new_string(self.value.clone()));
                client.write_integer(self.value.len() as i64, registry)
            }
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
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
