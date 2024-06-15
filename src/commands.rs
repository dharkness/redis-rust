mod expire;
mod input;
mod parser;
mod when;

mod append;
mod copy;
mod del;
mod exists;
mod get;
mod get_del;
mod get_range;
mod set;
mod set_multiple;
mod str_len;

pub use parser::{Parser, TryParse};

pub fn get_commands() -> [(&'static str, Box<dyn TryParse>); 10] {
    [
        ("APPEND", Box::new(append::AppendParser::new())),
        ("COPY", Box::new(copy::CopyParser::new())),
        ("DEL", Box::new(del::DelParser::new())),
        ("EXISTS", Box::new(exists::ExistsParser::new())),
        ("GET", Box::new(get::GetParser::new())),
        ("GETDEL", Box::new(get_del::GetDelParser::new())),
        ("GETRANGE", Box::new(get_range::GetRangeParser::new())),
        ("MSET", Box::new(set_multiple::SetMultipleParser::new())),
        ("SET", Box::new(set::SetParser::new())),
        ("STRLEN", Box::new(str_len::StrLenParser::new())),
    ]
}
