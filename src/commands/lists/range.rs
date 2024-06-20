use crate::commands::prelude::*;

struct Range {
    key: String,
    start: i64,
    end: i64,
}

impl Range {
    pub fn new(key: String, start: i64, end: i64) -> Self {
        Self { key, start, end }
    }
}

impl Apply for Range {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => {
                let range = clamp_range(list.len(), self.start, self.end);

                if range.is_empty() {
                    Ok(Response::EmptyList)
                } else {
                    Ok(Response::ListRef(&list[range]))
                }
            }
            IfKindResult::NotSet => Ok(Response::EmptyList),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct RangeParser {}

impl RangeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RangeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Range::new(
            input.next_string()?,
            input.next_i64()?,
            input.next_i64()?,
        )))
    }
}
