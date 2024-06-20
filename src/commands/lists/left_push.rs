use crate::commands::prelude::*;

pub struct LeftPush {
    key: String,
    values: Vec<String>,
    create: bool,
}

impl LeftPush {
    pub fn new(key: String, values: Vec<String>, create: bool) -> Self {
        Self {
            key,
            values,
            create,
        }
    }
}

impl Apply for LeftPush {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(ref mut list)) => {
                let mut values = self.values.clone();
                values.reverse();
                list.splice(0..0, values);
                Ok(Response::Usize(list.len()))
            }
            IfKindResult::NotSet => {
                if !self.create {
                    return Ok(Response::Usize(0));
                }
                let mut values = self.values.clone();
                values.reverse();
                store.set(&self.key, Value::from(values));
                Ok(Response::Usize(self.values.len()))
            }
            _ => Err(Error::WrongType),
        }
    }
}

pub struct LeftPushParser {}

impl LeftPushParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for LeftPushParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(LeftPush::new(
            input.next_string()?,
            input.rest()?,
            true,
        )))
    }
}
