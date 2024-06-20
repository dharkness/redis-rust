use crate::commands::prelude::*;

pub struct Set {
    key: String,
    index: i64,
    value: String,
}

impl Set {
    pub fn new(key: String, index: i64, value: String) -> Self {
        Self { key, index, value }
    }
}

impl Apply for Set {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => {
                let len = list.len() as i64;
                let index = if self.index < 0 {
                    len + self.index
                } else {
                    self.index
                };

                if 0 <= index && index < len {
                    list[index as usize].clone_from(&self.value);
                    Ok(Response::Ok)
                } else {
                    Ok(Response::Null)
                }
            }
            IfKindResult::NotSet => Err(Error::KeyNotFound),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct SetParser {}

impl SetParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let index = input.next_i64()?;
        let value = input.next_string()?;

        Ok(Box::new(Set::new(key, index, value)))
    }
}
