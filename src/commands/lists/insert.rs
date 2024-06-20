use crate::commands::prelude::*;

pub struct Insert {
    key: String,
    before: bool,
    pivot: String,
    value: String,
}

impl Insert {
    pub fn new(key: String, before: bool, pivot: String, value: String) -> Self {
        Self {
            key,
            before,
            pivot,
            value,
        }
    }
}

impl Apply for Insert {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(ref mut list)) => {
                if let Some(index) = list.iter().position(|x| x == &self.pivot) {
                    if self.before {
                        list.insert(index, self.value.clone());
                    } else {
                        list.insert(index + 1, self.value.clone());
                    }
                    Ok(Response::Usize(list.len()))
                } else {
                    Ok(Response::Zero)
                }
            }
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct InsertParser {}

impl InsertParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for InsertParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let before = match input.next_token()?.as_str() {
            "BEFORE" => true,
            "AFTER" => false,
            _ => return Err(Error::Syntax),
        };
        let pivot = input.next_string()?;
        let value = input.next_string()?;

        Ok(Box::new(Insert::new(key, before, pivot, value)))
    }
}
