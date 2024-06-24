use itertools::Itertools;

use crate::commands::prelude::*;

struct IsMemberMultiple {
    key: String,
    values: Vec<String>,
}

impl IsMemberMultiple {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for IsMemberMultiple {
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

pub struct IsMemberMultipleParser {}

impl IsMemberMultipleParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for IsMemberMultipleParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(IsMemberMultiple::new(
            input.next_string()?,
            input.rest()?,
        )))
    }
}
