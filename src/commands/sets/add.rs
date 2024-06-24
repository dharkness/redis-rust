use crate::commands::prelude::*;

struct Add {
    key: String,
    values: Vec<String>,
}

impl Add {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for Add {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
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

pub struct AddParser {}

impl AddParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for AddParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Add::new(input.next_string()?, input.rest()?)))
    }
}
