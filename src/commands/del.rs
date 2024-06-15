use super::prelude::*;

struct Del {
    keys: Vec<String>,
}

impl Del {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }
}

impl Command for Del {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        client.write_integer(
            self.keys.iter().filter(|key| store.remove(key)).count() as i64,
            registry,
        )
    }
}

pub struct DelParser {}

impl DelParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for DelParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        Ok(Box::new(Del::new(input.rest()?)))
    }
}
