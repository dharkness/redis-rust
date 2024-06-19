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
                let len = s.len() as i64;
                let mut start = if self.start < 0 {
                    len + self.start
                } else {
                    self.start
                };
                let mut end = 1 + if self.end < 0 {
                    len + self.end
                } else {
                    self.end
                };

                if start < 0 {
                    start = 0;
                } else if start > len {
                    start = len;
                }
                if end < 0 {
                    end = 0;
                } else if end > len {
                    end = len;
                }

                if end > start {
                    Ok(Response::BulkString(
                        s[start as usize..end as usize].to_string(),
                    ))
                } else {
                    Ok(Response::EmptyBulkString)
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
