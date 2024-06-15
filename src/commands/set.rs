use chrono::Utc;

use super::prelude::*;

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
            Expiration::Keep => (),
            Expiration::Never => {
                store.persist(&self.key);
            }
            Expiration::At(at) => {
                if at > Utc::now() {
                    store.expire_at(&self.key, &at);
                } else {
                    store.remove(&self.key);
                    return client.write_ok(registry);
                }
            }
        }

        if self.get {
            if let Some(previous) = store.set(&self.key, &self.value) {
                client.write_bulk_string(previous.as_str(), registry)
            } else {
                client.write_null(registry)
            }
        } else {
            store.set(&self.key, &self.value);
            client.write_ok(registry)
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

    fn try_when(set: &mut Set, token: &str, _: &mut Input) -> Result<(), String> {
        set.when = match token {
            "NX" => When::NotExists,
            "XX" => When::Exists,
            _ => panic!("unexpected token"),
        };
        Ok(())
    }

    fn try_get(set: &mut Set, _: &str, _: &mut Input) -> Result<(), String> {
        set.get = true;
        Ok(())
    }

    fn try_expire(set: &mut Set, token: &str, input: &mut Input) -> Result<(), String> {
        set.expire = Expiration::try_parse(token, input)?;
        Ok(())
    }
}

impl TryParse for SetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
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
