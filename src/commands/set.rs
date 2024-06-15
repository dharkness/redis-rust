use std::time::Duration;

use chrono::{DateTime, Utc};

use super::prelude::*;

struct Set {
    key: String,
    value: String,
    when: When,
    get: bool,
    expire: Expire,
}

impl Set {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            when: When::Always,
            get: false,
            expire: Expire::Never,
        }
    }
}

impl Apply for Set {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match self.when {
            When::Exists => {
                if !store.contains_key(&self.key) {
                    return client.write_null(registry);
                }
            }
            When::NotExists => {
                if store.contains_key(&self.key) {
                    return client.write_null(registry);
                }
            }
            When::Always => (),
        }

        match self.expire {
            Expire::Keep => (),
            Expire::Never => store.keep_forever(&self.key),
            Expire::At(at) => store.expire_at(&self.key, &at),
        }

        if let Some(previous) = store.set(&self.key, &self.value) {
            if self.get {
                return client.write_bulk_string(previous.as_str(), registry);
            }
        }

        client.write_ok(registry)
    }
}

pub struct SetParser {
    mutators: Mutators<Set>,
}

impl SetParser {
    pub fn new() -> Self {
        Self {
            mutators: vec![
                (vec!["NX", "XX"], SetParser::try_when),
                (vec!["GET"], SetParser::try_get),
                (
                    vec!["EX", "PX", "EXAT", "PXAT", "KEEPTTL"],
                    SetParser::try_expire,
                ),
            ],
        }
    }

    fn try_when(set: &mut Set, token: &String, _: &mut Input) -> Result<(), String> {
        set.when = match token.as_str() {
            "NX" => When::NotExists,
            "XX" => When::Exists,
            _ => panic!("unexpected token"),
        };
        Ok(())
    }

    fn try_get(set: &mut Set, _: &String, _: &mut Input) -> Result<(), String> {
        set.get = true;
        Ok(())
    }

    fn try_expire(set: &mut Set, token: &String, input: &mut Input) -> Result<(), String> {
        set.expire = Expire::try_parse(token, input)?;
        Ok(())
    }
}

impl TryParse for SetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;
        let value = input.next_string()?;

        Ok(Box::new(mutate(
            "SET",
            &self.mutators,
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

enum Expire {
    At(DateTime<Utc>),
    Keep,
    Never,
}

impl Expire {
    pub fn try_parse(token: &str, input: &mut Input) -> Result<Self, String> {
        let time = input.next_int()?;
        if time <= 0 {
            return Err("invalid SET time".to_string());
        }

        let at = match token {
            "EX" => Utc::now() + Duration::new(time as u64, 0),
            "PX" => {
                Utc::now()
                    + if time >= 1_000 {
                        Duration::new(time as u64 / 1_000, (time % 1_000) as u32 * 1_000_000)
                    } else {
                        Duration::new(0, time as u32 * 1_000_000)
                    }
            }
            "EXAT" => match DateTime::from_timestamp_millis(1_000 * time as i64) {
                Some(at) => at,
                _ => return Err("invalid unix timestamp".to_string()),
            },
            "PXAT" => match DateTime::from_timestamp_millis(time as i64) {
                Some(at) => at,
                _ => return Err("invalid unix timestamp".to_string()),
            },
            "KEEPTTL" => return Ok(Expire::Keep),
            _ => return Err("invalid expiration code".to_string()),
        };

        println!("expires at {}", at.format("%Y-%m-%d %H:%M:%S"));
        Ok(Expire::At(at))
    }
}
