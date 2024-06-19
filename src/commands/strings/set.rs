use chrono::Utc;

use crate::commands::prelude::*;
use crate::parse::Expiration;

struct Set {
    key: String,
    value: String,
    when: When,
    get: bool,
    expire: Expiration,
}

impl Set {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            when: When::Always,
            get: false,
            expire: Expiration::Never,
        }
    }
}

impl Apply for Set {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match self.when {
            When::Exists => {
                if !store.contains_key(&self.key) {
                    return Ok(Response::Null);
                }
            }
            When::NotExists => {
                if store.contains_key(&self.key) {
                    return Ok(Response::Null);
                }
            }
            When::Always => (),
        }

        let previous = if self.get {
            match store.get_and_remove_if_kind(Kind::String, &self.key) {
                IfKindResult::Matched(value) if value.is_string() => Some(value),
                IfKindResult::NotSet => None,
                _ => return Err(Error::WrongType),
            }
        } else {
            None
        };

        let removed = match self.expire {
            Expiration::Keep => false,
            Expiration::Never => {
                store.persist(&self.key);
                false
            }
            Expiration::At(at) => {
                if at > Utc::now() {
                    store.expire_at(&self.key, &at);
                    false
                } else {
                    store.remove(&self.key);
                    true
                }
            }
        };

        if !removed {
            store.set(&self.key, Value::from(self.value.clone()));
        }
        match previous {
            Some(value) => Ok(Response::Value(value)),
            None if self.get => Ok(Response::Null),
            _ => Ok(Response::Ok),
        }
    }
}

pub struct SetParser {
    options: Options<Set>,
}

impl SetParser {
    pub fn new() -> Self {
        Self {
            options: vec![
                (vec!["NX", "XX"], SetParser::try_when),
                (vec!["GET"], SetParser::try_get),
                (
                    vec!["EX", "PX", "EXAT", "PXAT", "KEEPTTL"],
                    SetParser::try_expire,
                ),
            ],
        }
    }

    fn try_when(set: &mut Set, token: &str, _: &mut Input) -> Result<(), Error> {
        set.when = match token {
            "NX" => When::NotExists,
            "XX" => When::Exists,
            _ => panic!("unexpected token"),
        };
        Ok(())
    }

    fn try_get(set: &mut Set, _: &str, _: &mut Input) -> Result<(), Error> {
        set.get = true;
        Ok(())
    }

    fn try_expire(set: &mut Set, token: &str, input: &mut Input) -> Result<(), Error> {
        set.expire = Expiration::try_parse(token, input)?;
        Ok(())
    }
}

impl TryParse for SetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let value = input.next_string()?;

        Ok(Box::new(parse_options(
            "SET",
            &self.options,
            input,
            Set::new(key, value),
        )?))
    }
}

enum When {
    Always,
    Exists,
    NotExists,
}
