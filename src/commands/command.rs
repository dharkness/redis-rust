use super::prelude::*;

struct Command {
    args: Vec<String>,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl Apply for Command {
    fn apply(
        &self,
        _store: &mut Store,
        client: &mut Client,
        registry: &Registry,
    ) -> io::Result<()> {
        if self.args.is_empty() {
            client.write_simple_error("missing subcommand", registry)
        } else {
            println!("docs");

            client.write(b"%0\r\n", registry)
            // return self.write(b"%2\r\n+PING\r\n%2\r\n+summary\r\n+Play ping-pong\r\n+group\r\n+connection\r\n+ECHO\r\n%2\r\n+summary\r\n+Hear yourself speak\r\n+group\r\n+connection\r\n", registry);
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        Ok(Box::new(Command::new(input.rest()?)))
    }
}
