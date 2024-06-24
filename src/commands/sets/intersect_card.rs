use crate::commands::prelude::*;
use crate::storage::{intersect, SetOp};

struct IntersectCard {
    from: Vec<String>,
    limit: usize,
}

impl IntersectCard {
    pub fn new(from: Vec<String>) -> Self {
        Self {
            from,
            limit: usize::MAX,
        }
    }
}

impl Apply for IntersectCard {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        let len = match intersect(store, &self.from, self.limit) {
            SetOp::New(members) => members.len(),
            SetOp::ValueRef(Value::Set(members)) => usize::min(members.len(), self.limit),
            SetOp::Empty => return Ok(Response::EmptySet),
            _ => return Err(Error::WrongType),
        };
        Ok(Response::Usize(len))
    }
}

pub struct IntersectCardParser {
    options: Options<IntersectCard>,
}

impl IntersectCardParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["LIMIT"], IntersectCardParser::try_limit)],
        }
    }

    fn try_limit(set: &mut IntersectCard, _: &str, input: &mut Input) -> Result<(), Error> {
        let limit = input.next_u64()? as usize;
        if limit > 0 {
            set.limit = limit;
        }
        Ok(())
    }
}

impl TryParse for IntersectCardParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let count = input.next_u64_min(1)? as usize;
        let keys = input.next_strings("SINTERCARD", "key", count)?;

        Ok(Box::new(parse_options(
            "SINTERCARD",
            &self.options,
            input,
            IntersectCard::new(keys),
        )?))
    }
}
