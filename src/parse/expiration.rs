use std::time::Duration;

use chrono::{DateTime, Utc};

use super::Input;

pub enum Expiration {
    At(DateTime<Utc>),
    Keep,
    Never,
}

impl Expiration {
    pub fn try_parse(token: &str, input: &mut Input) -> Result<Self, String> {
        if token == "KEEPTTL" {
            return Ok(Expiration::Keep);
        }

        let time = input.next_i64()?;
        if time <= 0 {
            return Err("invalid expire time".to_string());
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
                _ => return Err("invalid expire unix time seconds".to_string()),
            },
            "PXAT" => match DateTime::from_timestamp_millis(time) {
                Some(at) => at,
                _ => return Err("invalid expire unix time milliseconds".to_string()),
            },
            _ => return Err("invalid expiration code".to_string()),
        };

        println!("expires at {}", at.format("%Y-%m-%d %H:%M:%S"));
        Ok(Expiration::At(at))
    }
}
