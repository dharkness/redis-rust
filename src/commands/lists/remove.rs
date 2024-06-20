use crate::commands::prelude::*;

pub struct Remove {
    key: String,
    count: i64,
    value: String,
}

impl Remove {
    pub fn new(key: String, count: i64, value: String) -> Self {
        Self { key, count, value }
    }
}

impl Apply for Remove {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => {
                let removed = if self.count > 0 {
                    let mut count = self.count;
                    let mut removed = 0;
                    let mut i = 0;

                    while i < list.len() && count > 0 {
                        if list[i] == self.value {
                            list.remove(i);
                            count -= 1;
                            removed += 1;
                        } else {
                            i += 1;
                        }
                    }

                    removed
                } else {
                    let mut count = -self.count;
                    let mut removed = 0;
                    let mut i = list.len() as i64 - 1;

                    while i >= 0 && count > 0 {
                        if list[i as usize] == self.value {
                            list.remove(i as usize);
                            count -= 1;
                            removed += 1;
                        } else {
                            i -= 1;
                        }
                    }

                    removed
                };

                Ok(Response::Usize(removed))
            }
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct RemoveParser {}

impl RemoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RemoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let count = input.next_i64()?;
        let value = input.next_string()?;

        Ok(Box::new(Remove::new(
            key,
            if count == 0 { i64::MAX } else { count },
            value,
        )))
    }
}
