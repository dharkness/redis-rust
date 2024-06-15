mod append;
mod copy;
mod del;
mod exists;
mod get;
mod get_del;
mod get_range;
mod set;
mod set_multiple;
mod set_multiple_if_not_set;
mod str_len;

mod prelude {
    pub use std::io;
    
    pub use mio::Registry;
    
    pub use crate::parser::{Command, mutate, Mutators, TryParse};
    pub use crate::store::Store;
    pub use crate::client::Client;
    pub use crate::input::Input;
}

use crate::parser::TryParse;

pub fn get_commands() -> [(&'static str, Box<dyn TryParse>); 11] {
    [
        ("APPEND", Box::new(append::AppendParser::new())),
        ("COPY", Box::new(copy::CopyParser::new())),
        ("DEL", Box::new(del::DelParser::new())),
        ("EXISTS", Box::new(exists::ExistsParser::new())),
        ("GET", Box::new(get::GetParser::new())),
        ("GETDEL", Box::new(get_del::GetDelParser::new())),
        ("GETRANGE", Box::new(get_range::GetRangeParser::new())),
        ("MSET", Box::new(set_multiple::SetMultipleParser::new())),
        ("MSETNX", Box::new(set_multiple_if_not_set::SetMultipleIfNotSetParser::new())),
        ("SET", Box::new(set::SetParser::new())),
        ("STRLEN", Box::new(str_len::StrLenParser::new())),
    ]
}
