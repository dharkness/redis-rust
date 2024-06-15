use super::prelude::*;

const NONE: &[u8] = b"+none\r\n";
const STRING: &[u8] = b"+string\r\n";

struct Type {
    key: String,
}

impl Type {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Type {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if store.contains_key(&self.key) {
            client.write(STRING, registry)
        } else {
            client.write(NONE, registry)
        }
    }
}

pub struct TypeParser {}

impl TypeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for TypeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Type::new(input.next_string()?)))
    }
}
