use crate::commands::prelude::*;

use super::{End, parse_end};

struct PopMultiple {
    keys: Vec<String>,
    end: End,
    count: usize,
}

impl PopMultiple {
    pub fn new(keys: Vec<String>, end: End) -> Self {
        Self {
            keys,
            end,
            count: 1,
        }
    }
}

impl Apply for PopMultiple {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        for key in &self.keys {
            match store.get_mut_if_kind(Kind::List, key) {
                IfKindResult::Matched(Value::List(list)) => {
                    if list.is_empty() {
                        store.remove(key);
                        continue;
                    }

                    if self.count >= list.len() {
                        return Ok(Response::ValueList(vec![
                            Value::from(key.clone()),
                            store.remove(key).expect("found"),
                        ]));
                    } else {
                        return Ok(Response::ValueList(vec![
                            Value::from(key.clone()),
                            Value::from(match self.end {
                                End::Left => list.drain(0..self.count).collect(),
                                End::Right => list.split_off(list.len() - self.count),
                            }),
                        ]));
                    }
                }
                IfKindResult::NotSet => continue,
                _ => return Err(Error::WrongType),
            }
        }

        Ok(Response::Null)
    }
}

pub struct PopMultipleParser {
    options: Options<PopMultiple>,
}

impl PopMultipleParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["COUNT"], PopMultipleParser::try_count)],
        }
    }

    fn try_count(position: &mut PopMultiple, _token: &str, input: &mut Input) -> Result<(), Error> {
        let count = input.next_usize()?;

        if count == 0 {
            return Err(Error::Raw(b"-count should be greater than 0\r\n"));
        }

        position.count = count;
        Ok(())
    }
}

impl TryParse for PopMultipleParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key_count = input.next_u64_min(1)? as usize;
        let keys = input.next_strings("LMPOP", "key", key_count)?;
        let end = parse_end(&input.next_token()?)?;

        Ok(Box::new(parse_options(
            "LMPOP",
            &self.options,
            input,
            PopMultiple::new(keys, end),
        )?))
    }
}
