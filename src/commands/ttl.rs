use chrono::Utc;

use super::prelude::*;

struct TimeToLive {
    key: String,
}

impl TimeToLive {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for TimeToLive {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if !store.contains_key(&self.key) {
            client.write_integer(-2, registry)
        } else if let Some(at) = store.expires(&self.key) {
            client.write_integer(at.signed_duration_since(Utc::now()).num_seconds(), registry)
        } else {
            client.write_integer(-1, registry)
        }
    }
}

pub struct TimeToLiveParser {}

impl TimeToLiveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for TimeToLiveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(TimeToLive::new(input.next_string()?)))
    }
}
