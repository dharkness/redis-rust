use crate::parser::TryParse;

mod prelude {
    pub use std::io;

    pub use mio::Registry;

    pub use crate::input::Input;
    pub use crate::network::Client;
    pub use crate::parser::{Apply, Expiration, Options, parse_options, TryParse};
    pub use crate::store::Store;
}

mod append;
mod command;
mod copy;
mod del;
mod echo;
mod exists;
mod expire;
mod expire_at;
mod expire_time;
mod get;
mod get_del;
mod get_ex;
mod get_range;
mod keys;
mod p_expire;
mod p_expire_at;
mod p_expire_time;
mod p_ttl;
mod persist;
mod rename;
mod set;
mod set_multiple;
mod set_multiple_if_not_set;
mod str_len;
mod ttl;
mod r#type;

pub fn get_commands() -> [(&'static str, Box<dyn TryParse>); 26] {
    [
        ("APPEND", Box::new(append::AppendParser::new())),
        ("COMMAND", Box::new(command::CommandParser::new())),
        ("COPY", Box::new(copy::CopyParser::new())),
        ("DEL", Box::new(del::DelParser::new())),
        ("ECHO", Box::new(echo::EchoParser::new())),
        ("EXISTS", Box::new(exists::ExistsParser::new())),
        ("EXPIRE", Box::new(expire::ExpireParser::new())),
        ("EXPIREAT", Box::new(expire_at::ExpireAtParser::new())),
        ("EXPIRETIME", Box::new(expire_time::ExpireTimeParser::new())),
        ("GET", Box::new(get::GetParser::new())),
        ("GETDEL", Box::new(get_del::GetDelParser::new())),
        ("GETEX", Box::new(get_ex::GetExParser::new())),
        ("GETRANGE", Box::new(get_range::GetRangeParser::new())),
        ("KEYS", Box::new(keys::KeysParser::new())),
        ("MSET", Box::new(set_multiple::SetMultipleParser::new())),
        (
            "MSETNX",
            Box::new(set_multiple_if_not_set::SetMultipleIfNotSetParser::new()),
        ),
        ("PERSIST", Box::new(persist::PersistParser::new())),
        ("PEXPIRE", Box::new(p_expire::PExpireParser::new())),
        ("PEXPIREAT", Box::new(p_expire_at::PExpireAtParser::new())),
        (
            "PEXPIRETIME",
            Box::new(p_expire_time::PExpireTimeParser::new()),
        ),
        ("PTTL", Box::new(p_ttl::PTimeToLiveParser::new())),
        ("RENAME", Box::new(rename::RenameParser::new())),
        ("SET", Box::new(set::SetParser::new())),
        ("STRLEN", Box::new(str_len::StrLenParser::new())),
        ("TTL", Box::new(ttl::TimeToLiveParser::new())),
        ("TYPE", Box::new(r#type::TypeParser::new())),
    ]
}
