use super::prelude::*;

struct PExpireTime {
    key: String,
}

impl PExpireTime {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for PExpireTime {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if !store.contains_key(&self.key) {
            client.write_integer(-2, registry)
        } else if let Some(at) = store.expires(&self.key) {
            client.write_integer(at.timestamp_millis(), registry)
        } else {
            client.write_integer(-1, registry)
        }
    }
}

pub struct PExpireTimeParser {}

impl PExpireTimeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for PExpireTimeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(PExpireTime::new(input.next_string()?)))
    }
}
