use crate::commands::prelude::*;

pub struct RightPop {
    key: String,
    count: Option<usize>,
}

impl RightPop {
    pub fn new(key: String, count: Option<usize>) -> Self {
        Self { key, count }
    }
}

impl Apply for RightPop {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => match self.count {
                Some(count) => {
                    if count == 0 {
                        Ok(Response::List(Vec::new()))
                    } else if count >= list.len() {
                        Ok(Response::List(
                            store.remove(&self.key).expect("found").into_list(),
                        ))
                    } else {
                        Ok(Response::List(list.split_off(list.len() - count)))
                    }
                }
                None => {
                    if list.is_empty() {
                        Ok(Response::Null)
                    } else {
                        let value = list.remove(list.len() - 1);
                        if list.is_empty() {
                            store.remove(&self.key);
                        }
                        Ok(Response::BulkString(value))
                    }
                }
            },
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct RightPopParser {}

impl RightPopParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RightPopParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let count = if input.has_next() {
            Some(input.next_usize()?)
        } else {
            None
        };

        Ok(Box::new(RightPop::new(key, count)))
    }
}
