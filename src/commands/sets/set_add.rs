use crate::commands::prelude::*;

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
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match store.get_mut_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(ref mut members)) => {
                let mut added = 0;
                for value in &self.values {
                    if members.insert(value.clone()) {
                        added += 1;
                    }
                }
                Ok(Response::Usize(added))
            }
            IfKindResult::NotSet => {
                store.set(&self.key, Value::set_from_vec(&self.values));
                Ok(Response::Usize(self.values.len()))
            }
            _ => Err(Error::WrongType),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetAdd::new(input.next_string()?, input.rest()?)))
    }
}

enum When {
    Always,
    Exists,
    NotExists,
}
