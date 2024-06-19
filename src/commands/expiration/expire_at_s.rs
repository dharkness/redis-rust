use chrono::DateTime;

use crate::commands::prelude::*;

use super::{Expire, try_expiry};

pub struct ExpireAtSecsParser {
    options: Options<Expire>,
}

impl ExpireAtSecsParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], try_expiry)],
        }
    }
}

impl TryParse for ExpireAtSecsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let unix_time_seconds = input.next_i64()?;

        Ok(Box::new(parse_options(
            "EXPIREAT",
            &self.options,
            input,
            Expire::new(
                key,
                DateTime::from_timestamp_millis(1_000 * unix_time_seconds)
                    .ok_or(Error::ExpireTime)?,
            ),
        )?))
    }
}
