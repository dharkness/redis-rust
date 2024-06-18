use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::network::Error;

use super::Input;

pub enum Expiration {
    At(DateTime<Utc>),
    Keep,
    Never,
}

impl Expiration {
    pub fn try_parse(token: &str, input: &mut Input) -> Result<Self, Error> {
        if token == "KEEPTTL" {
            return Ok(Expiration::Keep);
        }

        let time = input.next_i64()?;
        if time <= 0 {
            return Err(Error::ExpireTime);
        }

        let at = match token {
            "EX" => Utc::now() + Duration::new(time as u64, 0),
            "PX" => {
                Utc::now()
                    + if time >= 1_000 {
                        Duration::new(time as u64 / 1_000, (time % 1_000) as u32 * 1_000_000)
                    } else {
                        Duration::new(0, time as u32 * 1_000_000)
                    }
            }
            "EXAT" => match DateTime::from_timestamp_millis(time * 1_000) {
                Some(at) => at,
                _ => return Err(Error::ExpireTime),
            },
            "PXAT" => match DateTime::from_timestamp_millis(time) {
                Some(at) => at,
                _ => return Err(Error::ExpireTime),
            },
            _ => return Err(Error::ExpireTime),
        };

        println!("expires at {}", at.format("%Y-%m-%d %H:%M:%S"));
        Ok(Expiration::At(at))
    }
}
