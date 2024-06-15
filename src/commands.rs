mod expire;
mod input;
mod parser;
mod when;

mod append;
mod get;
mod get_del;
mod set;
mod str_len;

pub use parser::{Parser, TryParse};

pub fn get_commands() -> [(&'static str, Box<dyn TryParse>); 5] {
    [
        ("APPEND", Box::new(append::AppendParser::new())),
        ("GET", Box::new(get::GetParser::new())),
        ("GETDEL", Box::new(get_del::GetDelParser::new())),
        ("SET", Box::new(set::SetParser::new())),
        ("STRLEN", Box::new(str_len::StrLenParser::new())),
    ]
}
