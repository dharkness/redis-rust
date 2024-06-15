use chrono::DateTime;

use super::expire::{Expire, ExpireParser};
use super::prelude::*;

pub struct PExpireAtParser {
    options: Options<Expire>,
}

impl PExpireAtParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], PExpireAtParser::try_expiry)],
        }
    }

    fn try_expiry(expire: &mut Expire, token: &String, input: &mut Input) -> Result<(), String> {
        ExpireParser::try_expiry(expire, token, input)
    }
}

impl TryParse for PExpireAtParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;
        let unix_time_milliseconds = input.next_int()?;

        Ok(Box::new(parse_options(
            "PEXPIREAT",
            &self.options,
            input,
            Expire::new(
                key,
                DateTime::from_timestamp_millis(unix_time_milliseconds)
                    .ok_or("invalid PEXPIREAT unix time seconds".to_string())?,
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
