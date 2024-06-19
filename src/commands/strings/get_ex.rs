use chrono::Utc;

use crate::commands::prelude::*;
use crate::parse::Expiration;

struct GetEx {
    key: String,
    expire: Expiration,
}

impl GetEx {
    pub fn new(key: String) -> Self {
        Self {
            key,
            expire: Expiration::Keep,
        }
    }
}

impl Apply for GetEx {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(s)) => {
                let result = Ok(Response::BulkString(s.clone()));

                match self.expire {
                    Expiration::Keep => (),
                    Expiration::Never => {
                        store.persist(&self.key);
                    }
                    Expiration::At(at) => {
                        if at > Utc::now() {
                            store.expire_at(&self.key, &at);
                        } else {
                            store.remove(&self.key);
                        }
                    }
                }

                result
            }
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct GetExParser {
    options: Options<GetEx>,
}

impl GetExParser {
    pub fn new() -> Self {
        Self {
            options: vec![(
                vec!["EX", "PX", "EXAT", "PXAT", "PERSIST"],
                GetExParser::try_expire,
            )],
        }
    }

    fn try_expire(get_ex: &mut GetEx, token: &str, input: &mut Input) -> Result<(), Error> {
        get_ex.expire = Expiration::try_parse(token, input)?;
        Ok(())
    }
}

impl TryParse for GetExParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;

        Ok(Box::new(parse_options(
            "GETEX",
            &self.options,
            input,
            GetEx::new(key),
        )?))
    }
}
