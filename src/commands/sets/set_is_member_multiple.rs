use crate::commands::prelude::*;

struct SetIsMemberMultiple {
    key: String,
    values: Vec<String>,
}

impl SetIsMemberMultiple {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for SetIsMemberMultiple {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => {
                client.write_string(format!("~{}\r\n", self.values.len()), registry)?;
                for member in &self.values {
                    client.write_integer(if members.contains(member) { 1 } else { 0 }, registry)?;
                }
                Ok(())
            }
            IfKindResult::NotSet => {
                client.write_string(format!("~{}\r\n", self.values.len()), registry)?;
                for _ in 0..self.values.len() {
                    client.write_zero(registry)?;
                }
                Ok(())
            }
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetIsMemberMultipleParser {}

impl SetIsMemberMultipleParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetIsMemberMultipleParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetIsMemberMultiple::new(
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
