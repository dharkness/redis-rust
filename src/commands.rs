use crate::parser::TryParse;

mod prelude {
    pub use std::io;

    pub use mio::Registry;

    pub use crate::input::Input;
    pub use crate::network::Client;
    pub use crate::parser::{Apply, mutate, Mutators, TryParse};
    pub use crate::store::Store;
}

mod append;
mod command;
mod copy;
mod del;
mod exists;
mod expire;
mod expire_at;
mod get;
mod get_del;
mod get_range;
mod set;
mod set_multiple;
mod set_multiple_if_not_set;
mod str_len;

pub fn get_commands() -> [(&'static str, Box<dyn TryParse>); 14] {
    [
        ("APPEND", Box::new(append::AppendParser::new())),
        ("COMMAND", Box::new(command::CommandParser::new())),
        ("COPY", Box::new(copy::CopyParser::new())),
        ("DEL", Box::new(del::DelParser::new())),
        ("EXISTS", Box::new(exists::ExistsParser::new())),
        ("EXPIRE", Box::new(expire::ExpireParser::new())),
        ("EXPIREAT", Box::new(expire_at::ExpireAtParser::new())),
        ("GET", Box::new(get::GetParser::new())),
        ("GETDEL", Box::new(get_del::GetDelParser::new())),
        ("GETRANGE", Box::new(get_range::GetRangeParser::new())),
        ("MSET", Box::new(set_multiple::SetMultipleParser::new())),
        (
            "MSETNX",
            Box::new(set_multiple_if_not_set::SetMultipleIfNotSetParser::new()),
        ),
        ("SET", Box::new(set::SetParser::new())),
        ("STRLEN", Box::new(str_len::StrLenParser::new())),
    ]
}
