use std::io;

use mio::Registry;

use crate::client::Client;
use super::input::Input;
use super::parser::{Command, TryParse};

struct Exists {
    keys: Vec<String>,
}

impl Exists {
    pub fn new(keys: Vec<String>) -> Self {
        Self{keys}
    }
}

impl Command for Exists {
    fn apply(&self, store: &mut crate::store::Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        client.write_integer(
            self.keys.iter().filter(|key| store.contains_key(key)).count() as i64, 
            registry,
        )
    }
}

pub struct ExistsParser {}

impl ExistsParser {
    pub fn new() -> Self {
        Self{}
    }
}

impl TryParse for ExistsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        Ok(Box::new(Exists::new(input.rest()?)))
    }
}
