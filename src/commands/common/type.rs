use crate::commands::prelude::*;

const NONE: Response = Response::Raw(b"+none\r\n");

const INTEGER: Response = Response::Raw(b"+integer\r\n");
const LIST: Response = Response::Raw(b"+list\r\n");
const SET: Response = Response::Raw(b"+set\r\n");
const STRING: Response = Response::Raw(b"+string\r\n");

struct Type {
    key: String,
}

impl Type {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Type {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get(&self.key) {
            Some(value) => match value.kind() {
                Kind::Integer => Ok(INTEGER),
                Kind::List => Ok(LIST),
                Kind::Set => Ok(SET),
                Kind::String => Ok(STRING),
            },
            None => Ok(NONE),
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
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Type::new(input.next_string()?)))
    }
}
