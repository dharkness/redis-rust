use super::prelude::*;

struct SetAdd {
    key: String,
    values: Vec<String>,
}

impl SetAdd {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for SetAdd {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match store.get_mut_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(ref mut members)) => {
                let mut added = 0;
                for value in &self.values {
                    if members.insert(value.clone()) {
                        added += 1;
                    }
                }
                client.write_integer(added, registry)
            }
            IfKindResult::NotSet => {
                store.set(&self.key, Value::new_set(&self.values));
                client.write_integer(self.values.len() as i64, registry)
            }
            _ => client.write_simple_error(WRONG_TYPE, registry),
        }
    }
}

pub struct SetAddParser {}

impl SetAddParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetAddParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetAdd::new(input.next_string()?, input.rest()?)))
    }
}

enum When {
    Always,
    Exists,
    NotExists,
}
