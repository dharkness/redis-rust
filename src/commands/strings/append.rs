use crate::commands::prelude::*;

struct Append {
    key: String,
    value: String,
}

impl Append {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Apply for Append {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(ref mut s)) => {
                s.push_str(&self.value);
                Ok(Response::Usize(s.len()))
            }
            IfKindResult::NotSet => {
                store.set(&self.key, Value::from(self.value.clone()));
                Ok(Response::Usize(self.value.len()))
            }
            _ => Err(Error::WrongType),
        }
    }
}

pub struct AppendParser {}

impl AppendParser {
    pub const fn new() -> Self {
        Self {}
    }
}

impl TryParse for AppendParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Append::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}
