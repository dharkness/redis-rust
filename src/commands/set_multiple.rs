use std::io;

use mio::Registry;

use crate::client::Client;
use super::input::Input;
use super::parser::{Command, TryParse};

struct SetMultiple {
    key_value_pairs: Vec<String>,
}

impl SetMultiple {
    pub fn new(key_value_pairs: Vec<String>) -> Self {
        Self{ key_value_pairs }
    }
}

impl Command for SetMultiple {
    fn apply(&self, store: &mut crate::store::Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if self.key_value_pairs.len() % 2 != 0 {
            client.write_simple_error("wrong number of MSET arguments", registry)
        } else {
            for i in (0..self.key_value_pairs.len()).step_by(2) {
                store.set(&self.key_value_pairs[i], &self.key_value_pairs[i + 1]);
            }
            client.write_ok(registry)
        }
    }
}

pub struct SetMultipleParser {}

impl SetMultipleParser {
    pub fn new() -> Self {
        Self{}
    }
}

impl TryParse for SetMultipleParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        Ok(Box::new(SetMultiple::new(input.rest()?)))
    }
}
