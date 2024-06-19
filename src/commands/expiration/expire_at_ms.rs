use chrono::DateTime;

use crate::commands::prelude::*;

use super::{Expire, try_expiry};

pub struct ExpireAtMillisParser {
    options: Options<Expire>,
}

impl ExpireAtMillisParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], try_expiry)],
        }
    }
}

impl TryParse for ExpireAtMillisParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let unix_time_milliseconds = input.next_i64()?;

        Ok(Box::new(parse_options(
            "ExpireAtMillis",
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
