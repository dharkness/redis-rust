use super::prelude::*;

struct SetMultipleIfNotSet {
    key_value_pairs: Vec<String>,
}

impl SetMultipleIfNotSet {
    pub fn new(key_value_pairs: Vec<String>) -> Self {
        Self { key_value_pairs }
    }
}

impl Command for SetMultipleIfNotSet {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if self.key_value_pairs.len() % 2 != 0 {
            client.write_simple_error("wrong number of MSET arguments", registry)
        } else {
            for i in (0..self.key_value_pairs.len()).step_by(2) {
                if store.contains_key(&self.key_value_pairs[i]) {
                    return client.write_integer(0, registry);
                }
            }
            for i in (0..self.key_value_pairs.len()).step_by(2) {
                store.set(&self.key_value_pairs[i], &self.key_value_pairs[i + 1]);
            }
            client.write_integer(1, registry)
        }
    }
}

pub struct SetMultipleIfNotSetParser {}

impl SetMultipleIfNotSetParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetMultipleIfNotSetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        Ok(Box::new(SetMultipleIfNotSet::new(input.rest()?)))
    }
}
