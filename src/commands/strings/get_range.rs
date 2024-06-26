use crate::commands::prelude::*;

struct GetRange {
    key: String,
    start: i64,
    end: i64,
}

impl GetRange {
    pub fn new(key: String, start: i64, end: i64) -> Self {
        Self { key, start, end }
    }
}

impl Apply for GetRange {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(s)) => {
                let range = clamp_range(s.len(), self.start, self.end);

                if range.is_empty() {
                    Ok(Response::EmptyBulkString)
                } else {
                    Ok(Response::BulkStringRef(&s[range]))
                }
            }
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct GetRangeParser {}

impl GetRangeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetRangeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(GetRange::new(
            input.next_string()?,
            input.next_i64()?,
            input.next_i64()?,
        )))
    }
}
