use std::collections::HashMap;
use std::io;

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

    pub fn try_next_input(
        &self,
        buffer: &String,
        start: &mut usize,
    ) -> Result<Option<Input>, String> {
        if buffer.is_empty() {
            return Ok(None);
        }
        if buffer.as_bytes()[*start] != b'*' {
            return Err("expected '*'".to_string());
        }

        let mut index = *start;
        if let Some(end) = buffer[index..].find("\r\n") {
            // println!("end: {}", end);
            let part = &buffer[index + 1..index + end];
            // println!("line: {}", line);
            let len = part
                .parse::<usize>()
                .map_err(|_| "invalid array length".to_string())?;
            let mut tokens = Vec::with_capacity(len);
            index += end + 2;
            for _ in 0..len {
                if let Some(value) = self.try_next_token(buffer, &mut index)? {
                    tokens.push(value);
                } else {
                    return Ok(None);
                }
            }

            *start = index;
            Ok(Some(Input::new(tokens)))
        } else {
            Ok(None)
        }
    }

    fn try_next_token(&self, buffer: &String, index: &mut usize) -> Result<Option<String>, String> {
        if buffer.as_bytes()[*index] != b'$' {
            return Err("expected '$'".to_string());
        }

        if let Some(end) = buffer[*index..].find("\r\n") {
            let part = &buffer[*index + 1..*index + end];
            let len = part
                .parse::<usize>()
                .map_err(|_| "invalid bulk string length".to_string())?;
            let start = *index + end + 2;
            if len > buffer.len() - (start + 2) {
                return Ok(None);
            }

            *index = start + len + 2;
            Ok(Some(buffer[start..start + len].to_string()))
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
