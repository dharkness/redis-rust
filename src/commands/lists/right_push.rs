use crate::commands::prelude::*;

pub struct RightPush {
    key: String,
    values: Vec<String>,
    create: bool,
}

impl RightPush {
    pub fn new(key: String, values: Vec<String>, create: bool) -> Self {
        Self {
            key,
            values,
            create,
        }
    }
}

impl Apply for RightPush {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(ref mut list)) => {
                list.extend(self.values.clone());
                Ok(Response::Usize(list.len()))
            }
            IfKindResult::NotSet => {
                if !self.create {
                    return Ok(Response::Usize(0));
                }
                store.set(&self.key, Value::from(self.values.clone()));
                Ok(Response::Usize(self.values.len()))
            }
            _ => Err(Error::WrongType),
        }
    }
}

pub struct RightPushParser {}

impl RightPushParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RightPushParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(RightPush::new(
            input.next_string()?,
            input.rest()?,
            true,
        )))
    }
}
