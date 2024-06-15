use std::time::Duration;

use chrono::Utc;

use super::expire::{Expire, ExpireParser};
use super::prelude::*;

pub struct PExpireParser {
    mutators: Mutators<Expire>,
}

impl PExpireParser {
    pub fn new() -> Self {
        Self {
            mutators: vec![(vec!["NX", "XX", "LT", "GT"], PExpireParser::try_expiry)],
        }
    }

    fn try_expiry(expire: &mut Expire, token: &String, input: &mut Input) -> Result<(), String> {
        ExpireParser::try_expiry(expire, token, input)
    }
}

impl TryParse for PExpireParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;
        let milliseconds = input.next_int()?;

        if milliseconds <= 0 {
            return Err("invalid PEXPIRE milliseconds".to_string());
        }

        Ok(Box::new(mutate(
            "PEXPIRE",
            &self.mutators,
            input,
            Expire::new(
                key,
                Utc::now()
                    + if milliseconds > 1_000 {
                        Duration::new(
                            milliseconds as u64 / 1_000,
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
