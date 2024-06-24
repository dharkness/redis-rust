use crate::network::Error;

pub mod index;
pub mod insert;
pub mod left_pop;
pub mod left_push;
pub mod left_push_exists;
pub mod len;
pub mod r#move;
pub mod pop_multiple;
pub mod position;
pub mod range;
pub mod remove;
pub mod right_pop;
pub mod right_push;
pub mod right_push_exists;
pub mod set;
pub mod trim;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum End {
    Left,
    Right,
}

fn parse_end(tokey: &str) -> Result<End, Error> {
    match tokey {
        "LEFT" => Ok(End::Left),
        "RIGHT" => Ok(End::Right),
        _ => Err(Error::Syntax),
    }
}
