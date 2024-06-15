use std::time::Duration;

use chrono::{DateTime, Utc};

use super::input::Input;

pub enum Expire {
    At(DateTime<Utc>),
    Keep,
    Never,
}

impl Expire {
    pub fn try_parse(token: &str, input: &mut Input) -> Result<Self, String> {
        let time = input.next_int()?;
        let at = match token {
            "EX" => {
                Utc::now() + Duration::new(time, 0)
            }
            "PX" => {
                let duration = if time >= 1_000 {
                    let secs = time / 1_000;
                    Duration::new(secs as u64, (time % 1_000) as u32 * 1_000_000)
                } else {
                    Duration::new(0, time as u32 * 1_000_000)
                };
                Utc::now() + duration
            }
            "EXAT" => {
                match DateTime::from_timestamp_millis(1_000 * time as i64) {
                    Some(at) => at,
                    _ => return Err("invalid unix timestamp".to_string()),
                }
            }
            "PXAT" => {
                match DateTime::from_timestamp_millis(time as i64) {
                    Some(at) => at,
                    _ => return Err("invalid unix timestamp".to_string()),
                }
            }
            "KEEPTTL" => return Ok(Expire::Keep),
            _ => return Err("invalid expiration code".to_string()),
        };

        println!("expires at {}", at.format("%Y-%m-%d %H:%M:%S"));
        Ok(Expire::At(at))
    }
}

pub trait Expires {
    fn expire(&mut self, expire: Expire) -> bool;
}
