use std::collections::HashMap;
use std::io;
use std::str::from_utf8;

use mio::Registry;

use crate::commands::get_commands;
use crate::input::Input;
use crate::network::Client;
use crate::store::Store;

pub struct Parser {
    parsers: HashMap<&'static str, Box<dyn TryParse>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            parsers: HashMap::from(get_commands()),
        }
    }

    pub fn try_next_input(&self, buffer: &[u8]) -> Result<Option<(Input, usize)>, String> {
        if buffer.is_empty() {
            return Ok(None);
        }
        if buffer[0] != b'*' {
            return Err("expected '*'".to_string());
        }

        if let Some(end) = find_cr_lf(buffer) {
            let len = parse_integer(&buffer[1..end])?;
            if len < 1 {
                return Err("invalid command length".to_string());
            }

            let mut tokens = Vec::with_capacity(len as usize);
            let mut index = end + 2;

            for _ in 0..len {
                if let Some((token, end)) = self.try_next_token(&buffer[index..])? {
                    index += end;
                    tokens.push(from_utf8(token).unwrap().to_string());
                } else {
                    return Ok(None);
                }
            }

            Ok(Some((Input::new(tokens), index)))
        } else {
            Ok(None)
        }
    }

    fn try_next_token<'a>(&self, buffer: &'a [u8]) -> Result<Option<(&'a [u8], usize)>, String> {
        if buffer.is_empty() {
            return Ok(None);
        }
        if buffer[0] != b'$' {
            return Err("expected '$'".to_string());
        }

        if let Some(end) = find_cr_lf(buffer) {
            let len = parse_integer(&buffer[1..end])?;
            if len < 0 {
                return Err("invalid bulk string length".to_string());
            }
            let start = end + 2;
            if len as usize > buffer.len() - (start + 2) {
                return Ok(None);
            }

            Ok(Some((
                &buffer[start..start + len as usize],
                start + len as usize + 2,
            )))
        } else {
            Ok(None)
        }
    }

    pub fn try_parse_command(&self, mut input: Input) -> Result<Box<dyn Command>, String> {
        let command = input.next_token()?;
        println!("command: {}", command);
        let parser = self
            .parsers
            .get(command.as_str())
            .ok_or("unknown command".to_string())?;

        parser.try_parse(&mut input)
    }
}

fn find_cr_lf(buffer: &[u8]) -> Option<usize> {
    for (i, chunk) in buffer.windows(2).enumerate() {
        if chunk == b"\r\n" {
            return Some(i);
        }
    }

    None
}

fn parse_integer(byte_slice: &[u8]) -> Result<i64, String> {
    let mut result: i64 = 0;
    let mut negative = false;

    for (i, &byte) in byte_slice.iter().enumerate() {
        if i == 0 && byte == b'-' {
            negative = true;
            continue;
        }
        if byte.is_ascii_digit() {
            let digit = (byte - b'0') as i64;
            result = result * 10 + digit;
        } else {
            return Err("invalid integer character".to_string());
        }
    }

    Ok(if negative { -result } else { result })
}

pub trait Command {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()>;
}

pub trait TryParse {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String>;
}

pub type Mutator<T> = fn(&mut T, &String, &mut Input) -> Result<(), String>;
pub type Mutators<T> = Vec<(Vec<&'static str>, Mutator<T>)>;

pub fn mutate<T>(
    command: &str,
    mutators: &Mutators<T>,
    input: &mut Input,
    mut target: T,
) -> Result<T, String> {
    let iter = &mut mutators.iter();

    while input.has_next() {
        let token = input.next_token().unwrap();

        while let Some((tokens, op)) = iter.next() {
            if tokens.contains(&token.as_str()) {
                op(&mut target, &token, input)?;
                break;
            }
        }
    }

    if input.has_next() {
        Err(format!(
            "unexpected {} token {}",
            command,
            input.next_token().unwrap()
        )
        .to_string())
    } else {
        Ok(target)
    }
}
