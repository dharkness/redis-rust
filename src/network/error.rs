#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Raw(&'static [u8]),
    String(String),
    Protocol,
    UnknownCommand(String),
    UnknownOption(String, String),
    DuplicateOption(String, String),
    MissingArgument(String, String),
    Syntax,
    Integer,
    ExpireTime,
    KeyNotFound,
    WrongType,
}
