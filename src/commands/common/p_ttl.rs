use chrono::Utc;

use crate::commands::prelude::*;

struct PTimeToLive {
    key: String,
}

impl PTimeToLive {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for PTimeToLive {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if !store.contains_key(&self.key) {
            Ok(Response::I64(-2))
        } else if let Some(at) = store.expires(&self.key) {
            Ok(Response::I64(
                at.signed_duration_since(Utc::now()).num_milliseconds(),
            ))
        } else {
            Ok(Response::I64(-1))
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(PTimeToLive::new(input.next_string()?)))
    }
}
