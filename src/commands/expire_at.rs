use chrono::DateTime;

use super::expire::{Expire, ExpireParser};
use super::prelude::*;

pub struct ExpireAtParser {
    mutators: Mutators<Expire>,
}

impl ExpireAtParser {
    pub fn new() -> Self {
        Self {
            mutators: vec![(vec!["NX", "XX", "LT", "GT"], ExpireAtParser::try_expiry)],
        }
    }

    fn try_expiry(expire: &mut Expire, token: &String, input: &mut Input) -> Result<(), String> {
        ExpireParser::try_expiry(expire, token, input)
    }
}

impl TryParse for ExpireAtParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;
        let unix_time_seconds = input.next_int()?;

        Ok(Box::new(mutate(
            "EXPIREAT",
            &self.mutators,
            input,
            Expire::new(
                key,
                DateTime::from_timestamp_millis(1_000 * unix_time_seconds)
                    .ok_or("invalid EXPIREAT unix time seconds".to_string())?,
            ),
        )?))
    }
}

enum Expiry {
    Ignore,
    Has,
    None,
    LessThan,
    GreaterThan,
}
