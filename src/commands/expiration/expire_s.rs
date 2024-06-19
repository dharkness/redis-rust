use std::time::Duration;

use chrono::Utc;

use crate::commands::prelude::*;

use super::{Expire, try_expiry};

pub struct ExpireSecsParser {
    options: Options<Expire>,
}

impl ExpireSecsParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["NX", "XX", "LT", "GT"], try_expiry)],
        }
    }
}

impl TryParse for ExpireSecsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let seconds = input.next_u64()?;

        Ok(Box::new(parse_options(
            "EXPIRE",
            &self.options,
            input,
            Expire::new(key, Utc::now() + Duration::new(seconds, 0)),
        )?))
    }
}
