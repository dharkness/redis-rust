use crate::commands::prelude::*;

use super::right_push::RightPush;

pub struct RightPushExistsParser {}

impl RightPushExistsParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RightPushExistsParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(RightPush::new(
            input.next_string()?,
            input.rest()?,
            false,
        )))
    }
}
