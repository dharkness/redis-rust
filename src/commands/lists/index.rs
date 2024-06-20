use crate::commands::prelude::*;

struct Index {
    key: String,
    index: i64,
}

impl Index {
    pub fn new(key: String, index: i64) -> Self {
        Self { key, index }
    }
}

impl Apply for Index {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => {
                let index = clamp(list.len(), self.index);

                if index < list.len() {
                    Ok(Response::BulkStringRef(&list[index]))
                } else {
                    Ok(Response::Null)
                }
            }
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct IndexParser {}

impl IndexParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for IndexParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Index::new(
            input.next_string()?,
            input.next_i64()?,
        )))
    }
}
