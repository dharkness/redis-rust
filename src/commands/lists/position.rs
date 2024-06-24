use itertools::Itertools;

use crate::commands::prelude::*;

pub struct Position {
    key: String,
    value: String,
    rank: i64,
    count: usize,
    max_len: usize,
}

impl Position {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            rank: 1,
            count: 1,
            max_len: usize::MAX,
        }
    }
}

impl Apply for Position {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => {
                if self.count == 1 {
                    match if self.rank > 0 {
                        list.iter()
                            .enumerate()
                            .take(self.max_len)
                            .filter(|(_, v)| **v == self.value)
                            .nth(self.rank as usize - 1)
                            .map(|(index, _)| index)
                    } else {
                        list.iter()
                            .rev()
                            .enumerate()
                            .take(self.max_len)
                            .filter(|(_, v)| **v == self.value)
                            .nth(-self.rank as usize - 1)
                            .map(|(index, _)| list.len() - 1 - index)
                    } {
                        Some(index) => Ok(Response::Usize(index)),
                        None => Ok(Response::Null),
                    }
                } else {
                    if self.rank > 0 {
                        Ok(Response::ValueList(
                            list.iter()
                                .enumerate()
                                .take(self.max_len)
                                .filter(|(_, v)| **v == self.value)
                                .skip(self.rank as usize - 1)
                                .take(self.count)
                                .map(|(index, _)| Value::from(index))
                                .collect_vec(),
                        ))
                    } else {
                        Ok(Response::ValueList(
                            list.iter()
                                .rev()
                                .enumerate()
                                .take(self.max_len)
                                .filter(|(_, v)| **v == self.value)
                                .skip(-self.rank as usize - 1)
                                .take(self.count)
                                .map(|(index, _)| Value::from(list.len() - 1 - index))
                                .collect_vec(),
                        ))
                    }
                }
            }
            IfKindResult::NotSet => Err(Error::KeyNotFound),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct PositionParser {
    options: Options<Position>,
}

impl PositionParser {
    pub fn new() -> Self {
        Self {
            options: vec![
                (vec!["RANK"], PositionParser::try_rank),
                (vec!["COUNT"], PositionParser::try_count),
                (vec!["MAXLEN"], PositionParser::try_max_len),
            ],
        }
    }

    fn try_rank(position: &mut Position, _token: &str, input: &mut Input) -> Result<(), Error> {
        let rank = input.next_i64()?;

        if rank == 0 {
            return Err(Error::Raw(b"-RANK can't be zero: use 1 to start from the first match, 2 from the second ... or use negative to start from the end of the list\r\n"));
        }

        position.rank = rank;
        Ok(())
    }

    fn try_count(position: &mut Position, _token: &str, input: &mut Input) -> Result<(), Error> {
        let count = input.next_usize()?;

        position.count = if count == 0 { usize::MAX } else { count };
        Ok(())
    }

    fn try_max_len(position: &mut Position, _token: &str, input: &mut Input) -> Result<(), Error> {
        let max_len = input.next_usize()?;

        position.max_len = if max_len == 0 { usize::MAX } else { max_len };
        Ok(())
    }
}

impl TryParse for PositionParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let key = input.next_string()?;
        let value = input.next_string()?;

        Ok(Box::new(parse_options(
            "LPOS",
            &self.options,
            input,
            Position::new(key, value),
        )?))
    }
}
