use crate::commands::prelude::*;

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
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if !match store.get_if_kind(Kind::Set, &self.from) {
            IfKindResult::Matched(Value::Set(members)) => {
                let contains = members.contains(&self.value);
                if self.from == self.to {
                    return Ok(Response::int_from_bool(contains));
                }
                contains
            }
            IfKindResult::NotSet => false,
            _ => return Err(Error::WrongType),
        } {
            return Ok(Response::Zero);
        }

        match store.get_mut_if_kind(Kind::Set, &self.to) {
            IfKindResult::Matched(Value::Set(ref mut members)) => {
                if !members.contains(&self.value) {
                    members.insert(self.value.clone());
                }
            }
            IfKindResult::NotSet => {
                store.set(&self.to, Value::set_from_vec(&[self.value.clone()]));
            }
            _ => return Err(Error::WrongType),
        };

        let mut result = store.get_mut_if_kind(Kind::Set, &self.from);
        let set = result.expect_mut("from set").expect_set_mut();

        if set.remove(&self.value) && set.is_empty() {
            store.remove(&self.from);
        }
        Ok(Response::One)
    }
}

pub struct SetMoveParser {}

impl SetMoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetMoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
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
