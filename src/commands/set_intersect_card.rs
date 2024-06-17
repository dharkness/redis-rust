use crate::storage::{intersect, Intersect};

use super::prelude::*;

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
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let intersection = match intersect(store, &self.from, self.limit) {
            Intersect::Set(members) => members,
            Intersect::SetRef(members) => {
                if members.len() > self.limit {
                    members.iter().take(self.limit).cloned().collect()
                } else {
                    members.clone()
                }
            }
            Intersect::Empty => return client.write_empty_set(registry),
            Intersect::WrongType => return client.write_simple_error(WRONG_TYPE, registry),
        };
        client.write_integer(intersection.len() as i64, registry)
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

    fn try_limit(set: &mut SetIntersectCard, _: &str, input: &mut Input) -> Result<(), String> {
        let limit = input.next_u64()? as usize;
        if limit > 0 {
            set.limit = limit;
        }
        Ok(())
    }
}

impl TryParse for SetIntersectCardParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let count = input.next_u64_min(1)? as usize;
        let keys = input.next_strings(count)?;

        Ok(Box::new(parse_options(
            "SINTERCARD",
            &self.options,
            input,
            SetIntersectCard::new(keys),
        )?))
    }
}
