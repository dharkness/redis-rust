use crate::commands::prelude::*;

struct Command {
    args: Vec<String>,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Apply for Command {
    fn apply<'a>(&self, _store: &'a mut Store) -> Result<Response<'a>, Error> {
        if self.args.is_empty() {
            Ok(Response::EmptyList)
        } else {
            Ok(Response::EmptyMap)
        }
    }
}

pub struct CommandParser {}

impl CommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for CommandParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Command::new(input.rest()?)))
    }
}
