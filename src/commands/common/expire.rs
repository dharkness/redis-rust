use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::commands::prelude::*;

pub struct Expire {
    key: String,
    at: DateTime<Utc>,
    expiry: Expiry,
}

impl Expire {
    pub fn new(key: String, at: DateTime<Utc>) -> Self {
        Self {
            key,
            at,
            expiry: Expiry::Ignore,
        }
    }
}

impl Apply for Expire {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if self.at <= Utc::now() {
            store.remove(&self.key);
            return client.write_integer(0, registry);
        }

        let allow = store.contains_key(&self.key)
            && match self.expiry {
                Expiry::Ignore => true,
                Expiry::None => store.expires(&self.key).is_none(),
                Expiry::Has => store.expires(&self.key).is_some(),
                Expiry::LessThan => store.expires(&self.key).is_some_and(|at| self.at < *at),
                Expiry::GreaterThan => store.expires(&self.key).is_some_and(|at| self.at > *at),
            };
        if !allow {
            return client.write_integer(0, registry);
        }

        store.expire_at(&self.key, &self.at);
        client.write_integer(1, registry)
    }
}

pub struct ExpireParser {
    options: Options<Expire>,
}

impl ExpireParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], ExpireParser::try_expiry)],
        }
    }

    pub fn try_expiry(expire: &mut Expire, token: &str, _: &mut Input) -> Result<(), String> {
        expire.expiry = match token {
            "NX" => Expiry::None,
            "XX" => Expiry::Has,
            "LT" => Expiry::LessThan,
            "GT" => Expiry::GreaterThan,
            _ => panic!("unexpected token"),
        };
        Ok(())
    }
}

impl TryParse for ExpireParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;
        let seconds = input.next_u64()?;

        Ok(Box::new(parse_options(
            "EXPIRE",
            &self.options,
            input,
            Expire::new(key, Utc::now() + Duration::new(seconds, 0)),
        )?))
    }
}

enum Expiry {
    Ignore,
    Has,
    None,
    LessThan,
    GreaterThan,
}
