use crate::commands::prelude::*;
use crate::storage::{intersect, SetOp};

struct SetIntersectCard {
    from: Vec<String>,
    limit: usize,
}

impl SetIntersectCard {
    pub fn new(from: Vec<String>) -> Self {
        Self {
            from,
            limit: usize::MAX,
        }
    }
}

impl Apply for SetIntersectCard {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        let intersection = match intersect(store, &self.from, self.limit) {
            SetOp::Set(members) => members,
            SetOp::SetRef(members) => {
                if members.len() > self.limit {
                    members.iter().take(self.limit).cloned().collect()
                } else {
                    members.clone()
                }
            }
            SetOp::Empty => return Ok(Response::EmptySet),
            SetOp::WrongType => return Err(Error::WrongType),
        };
        Ok(Response::Usize(intersection.len()))
    }
}

pub struct SetIntersectCardParser {
    options: Options<SetIntersectCard>,
}

impl SetIntersectCardParser {
    pub fn new() -> Self {
        Self {
            options: vec![(vec!["LIMIT"], SetIntersectCardParser::try_limit)],
        }
    }

    fn try_limit(set: &mut SetIntersectCard, _: &str, input: &mut Input) -> Result<(), Error> {
        let limit = input.next_u64()? as usize;
        if limit > 0 {
            set.limit = limit;
        }
        Ok(())
    }
}

impl TryParse for SetIntersectCardParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let count = input.next_u64_min(1)? as usize;
        let keys = input.next_strings("SINTERCARD", "key", count)?;

        Ok(Box::new(parse_options(
            "SINTERCARD",
            &self.options,
            input,
            SetIntersectCard::new(keys),
        )?))
    }
}
