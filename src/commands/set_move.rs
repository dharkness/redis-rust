use super::prelude::*;

struct SetMove {
    from: String,
    to: String,
    value: String,
}

impl SetMove {
    pub fn new(from: String, to: String, value: String) -> Self {
        Self { from, to, value }
    }
}

impl Apply for SetMove {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if !match store.get_if_kind(Kind::Set, &self.from) {
            IfKindResult::Matched(Value::Set(members)) => {
                let contains = members.contains(&self.value);
                if self.from == self.to {
                    return client.write_integer(if contains { 1 } else { 0 }, registry);
                }
                contains
            }
            IfKindResult::NotSet => false,
            _ => return client.write_simple_error(WRONG_TYPE, registry),
        } {
            return client.write_zero(registry);
        }

        match store.get_mut_if_kind(Kind::Set, &self.to) {
            IfKindResult::Matched(Value::Set(ref mut members)) => {
                if !members.contains(&self.value) {
                    members.insert(self.value.clone());
                }
            }
            IfKindResult::NotSet => {
                store.set(&self.to, Value::new_set_from_list(&[self.value.clone()]));
            }
            _ => return client.write_simple_error(WRONG_TYPE, registry),
        };

        let mut result = store.get_mut_if_kind(Kind::Set, &self.from);
        let set = result.expect_mut("from set").expect_set_mut();

        if set.remove(&self.value) && set.is_empty() {
            store.remove(&self.from);
        }
        client.write_integer(1, registry)
    }
}

pub struct SetMoveParser {}

impl SetMoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetMoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(SetMove::new(
            input.next_string()?,
            input.next_string()?,
            input.next_string()?,
        )))
    }
}

enum When {
    Always,
    Exists,
    NotExists,
}
