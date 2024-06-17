use crate::commands::prelude::*;
use crate::storage::Pattern;

struct Keys {
    pattern: String,
}

impl Keys {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }
}

impl Apply for Keys {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Ok(pattern) = Pattern::try_parse(&self.pattern) {
            client.write_array(&store.keys(&pattern), registry)
        } else {
            client.write_empty_array(registry)
        }
    }
}

pub struct KeysParser {}

impl KeysParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for KeysParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Keys::new(input.next_string()?)))
    }
}
