use itertools::Itertools;

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
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => Ok(Response::ValueList(
                self.values
                    .iter()
                    .map(|value| {
                        if members.contains(value) {
                            Value::from(1)
                        } else {
                            Value::from(0)
                        }
                    })
                    .collect_vec(),
            )),
            IfKindResult::NotSet => {
                Ok(Response::ValueList(vec![Value::from(0); self.values.len()]))
            }
            _ => Err(Error::WrongType),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetIsMemberMultiple::new(
            input.next_string()?,
            input.rest()?,
        )))
    }
}
