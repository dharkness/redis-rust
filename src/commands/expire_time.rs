use super::prelude::*;

struct ExpireTime {
    key: String,
}

impl ExpireTime {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for ExpireTime {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if !store.contains_key(&self.key) {
            client.write_integer(-2, registry)
        } else if let Some(at) = store.expires(&self.key) {
            client.write_integer(at.timestamp(), registry)
        } else {
            client.write_integer(-1, registry)
        }
    }
}

pub struct ExpireTimeParser {}

impl ExpireTimeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for ExpireTimeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(ExpireTime::new(input.next_string()?)))
    }
}
