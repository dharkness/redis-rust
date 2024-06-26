use std::str::from_utf8;

use crate::commands::CommandTree;
use crate::network::{Error, Response};
use crate::storage::Store;

use super::Input;

pub struct Parser {
    commands: CommandTree,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            commands: CommandTree::new(),
        }
    }

    pub fn try_next_input<'a>(
        &self,
        buffer: &'a [u8],
    ) -> Result<Option<(Input<'a>, usize)>, Error> {
        if buffer.is_empty() {
            return Ok(None);
        }
        if buffer[0] != b'*' {
            return Err(Error::Protocol);
        }

        if let Some(end) = find_cr_lf(buffer) {
            let len = parse_i64(&buffer[1..end])?;
            if len < 1 {
                return Err(Error::Integer);
            }

            let mut tokens = Vec::with_capacity(len as usize);
            let mut index = end + 2;

            for _ in 0..len {
                if let Some((token, end)) = self.try_next_token(&buffer[index..])? {
                    index += end;
                    tokens.push(token);
                } else {
                    return Ok(None);
                }
            }

            Ok(Some((Input::new(tokens), index)))
        } else {
            Ok(None)
        }
    }

    fn try_next_token<'a>(&self, buffer: &'a [u8]) -> Result<Option<(&'a str, usize)>, Error> {
        if buffer.is_empty() {
            return Ok(None);
        }
        if buffer[0] != b'$' {
            return Err(Error::Protocol);
        }

        if let Some(end) = find_cr_lf(buffer) {
            let len = parse_i64(&buffer[1..end])?;
            if len < 0 {
                return Err(Error::Protocol);
            }
            let start = end + 2;
            if len as usize > buffer.len() - (start + 2) {
                return Ok(None);
            }

            Ok(Some((
                from_utf8(&buffer[start..start + len as usize]).map_err(|_| Error::Protocol)?,
                start + len as usize + 2,
            )))
        } else {
            Ok(None)
        }
    }

    pub fn try_parse_command(&self, mut input: Input) -> Result<Box<dyn Apply>, Error> {
        let command = input.next()?;
        println!("command: {}", command);
        let parser = self
            .commands
            .get(command)
            .ok_or(Error::UnknownCommand(command.to_string()))?;

        parser.try_parse(&mut input).and_then(|parsed| {
            if input.has_next() {
                Err(Error::Syntax)
            } else {
                Ok(parsed)
            }
        })
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

pub fn parse_i64(buffer: &[u8]) -> Result<i64, Error> {
    let mut result: i64 = 0;
    let mut negative = false;

    for (i, &byte) in buffer.iter().enumerate() {
        if i == 0 && byte == b'-' {
            negative = true;
            continue;
        }
        if byte.is_ascii_digit() {
            result = result * 10 + (byte - b'0') as i64;
        } else {
            return Err(Error::Integer);
        }
    }

    Ok(if negative { -result } else { result })
}

pub fn parse_u64(buffer: &[u8]) -> Result<u64, Error> {
    let mut result: u64 = 0;

    for byte in buffer.iter() {
        if byte.is_ascii_digit() {
            result = result * 10 + (byte - b'0') as u64;
        } else {
            return Err(Error::Integer);
        }
    }

    Ok(result)
}

pub trait Apply {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error>;
}

pub trait TryParse {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error>;
}
