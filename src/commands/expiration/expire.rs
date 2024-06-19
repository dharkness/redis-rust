use chrono::{DateTime, Utc};

use crate::commands::prelude::*;

pub struct Expire {
    key: String,
    at: DateTime<Utc>,
    expiry: When,
}

impl Expire {
    pub fn new(key: String, at: DateTime<Utc>) -> Self {
        Self {
            key,
            at,
            expiry: When::Always,
        }
    }
}

impl Apply for Expire {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        let allow = store.contains_key(&self.key)
            && match self.expiry {
                When::Always => true,
                When::None => store.expires(&self.key).is_none(),
                When::Has => store.expires(&self.key).is_some(),
                When::LessThan => store.expires(&self.key).is_some_and(|at| self.at < *at),
                When::GreaterThan => store.expires(&self.key).is_some_and(|at| self.at > *at),
            };
        if !allow {
            return Ok(Response::Zero);
        }

        if self.at <= Utc::now() {
            store.remove(&self.key);
            return Ok(Response::Zero);
        } else {
            store.expire_at(&self.key, &self.at);
            Ok(Response::One)
        }
    }
}

enum When {
    Always,
    Has,
    None,
    LessThan,
    GreaterThan,
}

pub fn try_expiry(expire: &mut Expire, token: &str, _input: &mut Input) -> Result<(), Error> {
    expire.expiry = match token {
        "NX" => When::None,
        "XX" => When::Has,
        "LT" => When::LessThan,
        "GT" => When::GreaterThan,
        _ => {
            return Err(Error::UnknownOption(
                "EXPIRE".to_string(),
                token.to_string(),
            ))
        }
    };
    Ok(())
}
