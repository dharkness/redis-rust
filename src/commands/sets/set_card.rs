use crate::commands::prelude::*;

struct SetCard {
    key: String,
}

impl SetCard {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for SetCard {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => {
                client.write_integer(members.len() as i64, registry)
            }
            IfKindResult::NotSet => client.write_zero(registry),
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetCardParser {}

impl SetCardParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetCardParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetCard::new(input.next_string()?)))
    }
}
