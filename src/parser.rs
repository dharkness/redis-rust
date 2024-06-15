use std::collections::HashMap;
use std::io;
use std::str::from_utf8;

use mio::Registry;

use crate::commands::get_commands;
use crate::input::Input;
use crate::network::Client;
use crate::store::Store;

pub struct Parser {
    commands: CommandTree,
}

impl Parser {
    pub fn new() -> Self {
        let mut commands = CommandTree::new();
        for (name, parser) in get_commands() {
            commands.insert(name, parser);
        }

        Self { commands }
    }

    pub fn try_next_input<'a>(
        &self,
        buffer: &'a [u8],
    ) -> Result<Option<(Input<'a>, usize)>, String> {
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

    fn try_next_token<'a>(&self, buffer: &'a [u8]) -> Result<Option<(&'a str, usize)>, String> {
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
                from_utf8(&buffer[start..start + len as usize]).map_err(|e| e.to_string())?,
                start + len as usize + 2,
            )))
        } else {
            Ok(None)
        }
    }

    pub fn try_parse_command(&self, mut input: Input) -> Result<Box<dyn Apply>, String> {
        let command = input.next()?;
        println!("command: {}", command);
        let parser = self
            .commands
            .get(command)
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

pub fn parse_integer(buffer: &[u8]) -> Result<i64, String> {
    let mut result: i64 = 0;
    let mut negative = false;

    for (i, &byte) in buffer.iter().enumerate() {
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

pub trait Apply {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()>;
}

pub trait TryParse {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String>;
}

pub type ParseOption<T> = fn(&mut T, &String, &mut Input) -> Result<(), String>;
pub type Options<T> = Vec<(Vec<&'static str>, ParseOption<T>)>;

pub fn parse_options<T>(
    command: &str,
    options: &Vec<(Vec<&'static str>, ParseOption<T>)>,
    input: &mut Input,
    mut target: T,
) -> Result<T, String> {
    let mut used_ops = Vec::new();

    'outer: while input.has_next() {
        let token = input.next_token().unwrap();

        println!("token: {}", token);
        for (ref tokens, ref op) in options {
            println!("tokens: {:?}", tokens);
            if tokens.contains(&token.as_str()) {
                if used_ops.contains(&op) {
                    return Err(format!("duplicate {} option {}", command, token));
                }

                used_ops.push(op);
                op(&mut target, &token, input)?;
                continue 'outer;
            }
        }

        return Err(format!("unexpected {} token {}", command, token));
    }

    Ok(target)
}

struct CommandTree {
    parser: Option<Box<dyn TryParse>>,
    children: HashMap<char, CommandTree>,
}

impl CommandTree {
    fn new() -> Self {
        Self {
            parser: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, command: &str, parser: Box<dyn TryParse>) {
        let mut current = self;

        for c in command.chars() {
            current = current
                .children
                .entry(c.to_ascii_uppercase())
                .or_insert(CommandTree::new());
        }

        current.parser = Some(parser);
    }

    fn get(&self, command: &str) -> Option<&Box<dyn TryParse>> {
        let mut current = self;

        for c in command.chars() {
            if let Some(next) = current.children.get(&(c.to_ascii_uppercase())) {
                current = next;
            } else {
                return None;
            }
        }

        current.parser.as_ref()
    }
}
