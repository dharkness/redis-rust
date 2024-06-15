use std::io;

use mio::Registry;

use crate::client::Client;
use super::input::Input;
use super::parser::{Command, TryParse};

struct StrLen {
    key: String,
}

impl StrLen {
    pub fn new(key: String) -> Self {
        Self{key}
    }
}

impl Command for StrLen {
    fn apply(&self, store: &mut crate::store::Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Some(value) = store.get(&self.key) {
            client.write_integer(value.len() as u64, registry)
        } else {
            client.write_integer(0, registry)
        }
    }
}

pub struct StrLenParser {}

impl StrLenParser {
    pub fn new() -> Self {
        Self{}
    }
}

impl TryParse for StrLenParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        Ok(Box::new(StrLen::new(input.next()?)))
    }
}
