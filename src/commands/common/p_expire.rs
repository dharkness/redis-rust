use std::time::Duration;

use chrono::Utc;

use crate::commands::prelude::*;

use super::expire::{Expire, ExpireParser};

pub struct PExpireParser {
    options: Options<Expire>,
}

impl PExpireParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], PExpireParser::try_expiry)],
        }
    }

    fn try_expiry(expire: &mut Expire, token: &str, input: &mut Input) -> Result<(), Error> {
        ExpireParser::try_expiry(expire, token, input)
    }
}

impl TryParse for PExpireParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let milliseconds = input.next_u64()?;

        Ok(Box::new(parse_options(
            "PEXPIRE",
            &self.options,
            input,
            Expire::new(
                key,
                Utc::now()
                    + if milliseconds > 1_000 {
                        Duration::new(
                            milliseconds / 1_000,
                            (milliseconds % 1_000) as u32 * 1_000_000,
                        )
                    } else {
                        Duration::new(0, milliseconds as u32 * 1_000_000)
                    },
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
