use crate::commands::prelude::*;

use super::left_push::LeftPush;

pub struct LeftPushExistsParser {}

impl LeftPushExistsParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for LeftPushExistsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(LeftPush::new(
            input.next_string()?,
            input.rest()?,
            false,
        )))
    }
}
