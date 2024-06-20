use crate::commands::prelude::*;

struct Trim {
    key: String,
    start: i64,
    end: i64,
}

impl Trim {
    pub fn new(key: String, start: i64, end: i64) -> Self {
        Self { key, start, end }
    }
}

impl Apply for Trim {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => {
                let range = clamp_range(list.len(), self.start, self.end);

                if range.is_empty() {
                    store.remove(&self.key);
                } else {
                    list.drain(0..range.start);
                    list.drain(range.end - range.start..);
                }
                Ok(Response::Ok)
            }
            IfKindResult::NotSet => Ok(Response::Ok),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct TrimParser {}

impl TrimParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for TrimParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Trim::new(
            input.next_string()?,
            input.next_i64()?,
            input.next_i64()?,
        )))
    }
}
