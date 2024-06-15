use std::io;
use std::ops::Add;

use mio::Registry;

use crate::client::Client;
use super::input::Input;
use super::parser::{Command, TryParse};

struct Append {
    key: String,
    value: String,
}

impl Append {
    pub fn new(key: String, value: String) -> Self {
        Self{key, value}
    }
}

impl Command for Append {
    fn apply(&self, store: &mut crate::store::Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let new_value = if let Some(current) = store.get(&self.key) {
            current.clone().add(&self.value)
        } else {
            self.value.clone()
        };
        store.set(&self.key, &new_value);
        client.write_integer(new_value.len() as u64, registry)
    }
}

pub struct AppendParser {}

impl AppendParser {
    pub const fn new() -> Self {
        Self{}
    }
}

impl TryParse for AppendParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        Ok(Box::new(Append::new(input.next()?, input.next()?)))
    }
}
