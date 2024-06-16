use super::prelude::*;

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
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Some(value) = store.get(&self.key) {
            let result = client.write_bulk_string(value, registry);

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
        } else {
            client.write_null(registry)
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

    fn try_expire(get_ex: &mut GetEx, token: &str, input: &mut Input) -> Result<(), String> {
        get_ex.expire = Expiration::try_parse(token, input)?;
        Ok(())
    }
}

impl TryParse for GetExParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;

        Ok(Box::new(parse_options(
            "GETEX",
            &self.options,
            input,
            GetEx::new(key),
        )?))
    }
}
