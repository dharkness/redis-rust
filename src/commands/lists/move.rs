use crate::commands::prelude::*;

struct Move {
    from: String,
    from_end: End,
    to: String,
    to_end: End,
}

impl Move {
    pub fn new(from: String, from_end: End, to: String, to_end: End) -> Self {
        Self {
            from,
            from_end,
            to,
            to_end,
        }
    }
}

impl Apply for Move {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        if store.is_not_kind(&self.to, Kind::List) {
            return Err(Error::WrongType);
        }

        let value = match store.get_mut_if_kind(Kind::List, &self.from) {
            IfKindResult::Matched(Value::List(list)) => {
                if list.is_empty() {
                    return Ok(Response::Null);
                }

                if self.from == self.to {
                    return Ok(Response::BulkString(
                        if list.len() == 1 {
                            &list[0]
                        } else {
                            match (self.from_end, self.to_end) {
                                (End::Left, End::Left) => &list[0],
                                (End::Right, End::Right) => &list[list.len() - 1],
                                (End::Left, End::Right) => {
                                    list.rotate_left(1);
                                    &list[list.len() - 1]
                                }
                                (End::Right, End::Left) => {
                                    list.rotate_right(1);
                                    &list[0]
                                }
                            }
                        }
                        .clone(),
                    ));
                }

                match self.from_end {
                    End::Left => list.remove(0),
                    End::Right => list.pop().unwrap(),
                }
            }
            IfKindResult::NotSet => return Ok(Response::Null),
            _ => return Err(Error::WrongType),
        };

        match store.get_mut_if_kind(Kind::List, &self.to) {
            IfKindResult::Matched(Value::List(list)) => match self.to_end {
                End::Left => list.insert(0, value.clone()),
                End::Right => list.push(value.clone()),
            },
            IfKindResult::NotSet => {
                store.set(&self.to, Value::from(vec![value.clone()]));
            }
            _ => return Err(Error::WrongType),
        }

        Ok(Response::BulkString(value))
    }
}

pub struct MoveParser {}

impl MoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for MoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        let from = input.next_string()?;
        let to = input.next_string()?;

        Ok(Box::new(Move::new(
            from,
            parse_end(input)?,
            to,
            parse_end(input)?,
        )))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum End {
    Left,
    Right,
}

fn parse_end(input: &mut Input) -> Result<End, Error> {
    match input.next_token()?.as_str() {
        "LEFT" => Ok(End::Left),
        "RIGHT" => Ok(End::Right),
        _ => Err(Error::Syntax),
    }
}
