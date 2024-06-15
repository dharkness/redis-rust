use chrono::Utc;

use super::prelude::*;

struct PTimeToLive {
    key: String,
}

impl PTimeToLive {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for PTimeToLive {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if !store.contains_key(&self.key) {
            client.write_integer(-2, registry)
        } else if let Some(at) = store.expires(&self.key) {
            client.write_integer(
                at.signed_duration_since(Utc::now()).num_milliseconds(),
                registry,
            )
        } else {
            client.write_integer(-1, registry)
        }
    }
}

pub struct PTimeToLiveParser {}

impl PTimeToLiveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for PTimeToLiveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(PTimeToLive::new(input.next_string()?)))
    }
}
