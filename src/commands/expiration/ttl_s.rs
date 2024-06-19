use chrono::Utc;

use crate::commands::prelude::*;

struct TimeToLive {
    key: String,
}

impl TimeToLive {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for TimeToLive {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        if !store.contains_key(&self.key) {
            Ok(Response::I64(-2))
        } else if let Some(at) = store.expires(&self.key) {
            Ok(Response::I64(
                at.signed_duration_since(Utc::now()).num_seconds(),
            ))
        } else {
            Ok(Response::I64(-1))
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(TimeToLive::new(input.next_string()?)))
    }
}
