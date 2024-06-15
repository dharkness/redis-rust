mod expire;
mod input;
mod parser;
mod when;

mod append;
mod exists;
mod get;
mod get_del;
mod get_range;
mod set;
mod str_len;

pub use parser::{Parser, TryParse};

pub fn get_commands() -> [(&'static str, Box<dyn TryParse>); 7] {
    [
        ("APPEND", Box::new(append::AppendParser::new())),
        ("EXISTS", Box::new(exists::ExistsParser::new())),
        ("GET", Box::new(get::GetParser::new())),
        ("GETDEL", Box::new(get_del::GetDelParser::new())),
        ("GETRANGE", Box::new(get_range::GetRangeParser::new())),
        ("SET", Box::new(set::SetParser::new())),
        ("STRLEN", Box::new(str_len::StrLenParser::new())),
    ]
}
