use super::prelude::*;

struct Copy {
    source: String,
    destination: String,
    replace: bool,
}

impl Copy {
    pub fn new(source: String, destination: String) -> Self {
        Self {
            source,
            destination,
            replace: false,
        }
    }
}

impl Apply for Copy {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if (self.replace || !store.contains_key(&self.destination))
            && store.copy(&self.source, &self.destination)
        {
            client.write_integer(1, registry)
        } else {
            client.write_integer(0, registry)
        }
    }
}

pub struct CopyParser {
    options: Options<Copy>,
}

impl CopyParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["REPLACE"], CopyParser::try_replace)],
        }
    }

    fn try_replace(copy: &mut Copy, _: &String, _: &mut Input) -> Result<(), String> {
        copy.replace = true;
        Ok(())
    }
}

impl TryParse for CopyParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let source = input.next_string()?;
        let destination = input.next_string()?;

        Ok(Box::new(parse_options(
            "COPY",
            &self.options,
            input,
            Copy::new(source, destination),
        )?))
    }
}
