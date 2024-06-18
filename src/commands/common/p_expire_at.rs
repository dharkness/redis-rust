use chrono::DateTime;

use crate::commands::prelude::*;

use super::expire::{Expire, ExpireParser};

pub struct PExpireAtParser {
    options: Options<Expire>,
}

impl PExpireAtParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], PExpireAtParser::try_expiry)],
        }
    }

    fn try_expiry(expire: &mut Expire, token: &str, input: &mut Input) -> Result<(), Error> {
        ExpireParser::try_expiry(expire, token, input)
    }
}

impl TryParse for PExpireAtParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let unix_time_milliseconds = input.next_i64()?;

        Ok(Box::new(parse_options(
            "PEXPIREAT",
            &self.options,
            input,
            Expire::new(
                key,
                DateTime::from_timestamp_millis(unix_time_milliseconds).ok_or(Error::ExpireTime)?,
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
