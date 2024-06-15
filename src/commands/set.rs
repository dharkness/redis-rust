use std::io;

use mio::Registry;

use crate::client::Client;
use super::expire::Expire;
use super::input::Input;
use super::parser::{Command, mutate, Mutators, TryParse};
use super::when::When;

struct Set {
    key: String,
    value: String,
    when: When,
    get: bool,
    expire: Expire,
}

impl Set {
    pub fn new(key: String, value: String) -> Self {
        Self{key, value, when: When::Always, get: false, expire: Expire::Never}
    }
}

impl Command for Set {
    fn apply(&self, store: &mut crate::store::Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        match self.when {
            When::Exists => if !store.contains_key(&self.key) {
                return client.write_null(registry);
            },
            When::NotExists => if store.contains_key(&self.key) {
                return client.write_null(registry);
            },
            When::Always => (),
        }

        let mut result = None;
        if self.get {
            result = store.get(&self.key).map(|value| value.clone());
        }

        match self.expire {
            Expire::Keep => (),
            Expire::Never => store.keep_forever(&self.key),
            Expire::At(at) => store.expire_at(&self.key, &at),
        }
        store.set(&self.key, &self.value);

        if let Some(value) = result {
            client.write_bulk_string(&value, registry)
        } else {
            client.write_ok(registry)
        }
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
                (vec!["EX", "PX", "EXAT", "PXAT", "KEEPTTL"], SetParser::try_expire),
            ]
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        let key = input.next()?;
        let value = input.next()?;

        Ok(Box::new(mutate("SET", &self.mutators, input, Set::new(key, value))?))
    }
}
