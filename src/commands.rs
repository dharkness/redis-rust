use std::collections::HashMap;

use crate::parse::TryParse;

mod prelude {
    pub use crate::network::*;
    pub use crate::parse::{Apply, Input, Options, parse_options, TryParse};
    pub use crate::storage::{IfKindResult, Kind, Store, Value};
}

mod common;
mod expiration;
mod server;
mod sets;
mod strings;

fn get_commands() -> [(&'static str, Box<dyn TryParse>); 42] {
    [
        //
        // server
        //
        ("COMMAND", Box::new(server::command::CommandParser::new())),
        ("ECHO", Box::new(server::echo::EchoParser::new())),
        //
        // common
        //
        ("COPY", Box::new(common::copy::CopyParser::new())),
        ("DEL", Box::new(common::del::DelParser::new())),
        ("EXISTS", Box::new(common::exists::ExistsParser::new())),
        ("KEYS", Box::new(common::keys::KeysParser::new())),
        ("RENAME", Box::new(common::rename::RenameParser::new())),
        ("TYPE", Box::new(common::r#type::TypeParser::new())),
        //
        // expiration
        //
        (
            "EXPIRE",
            Box::new(expiration::expire_s::ExpireSecsParser::new()),
        ),
        (
            "EXPIREAT",
            Box::new(expiration::expire_at_s::ExpireAtSecsParser::new()),
        ),
        (
            "EXPIRETIME",
            Box::new(expiration::expire_time_s::ExpireTimeSecsParser::new()),
        ),
        (
            "PERSIST",
            Box::new(expiration::persist::PersistParser::new()),
        ),
        (
            "PEXPIRE",
            Box::new(expiration::expire_ms::ExpireMillisParser::new()),
        ),
        (
            "ExpireAtMillis",
            Box::new(expiration::expire_at_ms::ExpireAtMillisParser::new()),
        ),
        (
            "PEXPIRETIME",
            Box::new(expiration::expire_time_ms::ExpireTimeMillisParser::new()),
        ),
        (
            "PTTL",
            Box::new(expiration::ttl_ms::PTimeToLiveParser::new()),
        ),
        ("TTL", Box::new(expiration::ttl_s::TimeToLiveParser::new())),
        //
        // strings
        //
        ("APPEND", Box::new(strings::append::AppendParser::new())),
        ("GET", Box::new(strings::get::GetParser::new())),
        ("GETDEL", Box::new(strings::get_del::GetDelParser::new())),
        ("GETEX", Box::new(strings::get_ex::GetExParser::new())),
        (
            "GETRANGE",
            Box::new(strings::get_range::GetRangeParser::new()),
        ),
        (
            "MSET",
            Box::new(strings::set_multiple::SetMultipleParser::new()),
        ),
        (
            "MSETNX",
            Box::new(strings::set_multiple_if_not_set::SetMultipleIfNotSetParser::new()),
        ),
        ("SET", Box::new(strings::set::SetParser::new())),
        ("STRLEN", Box::new(strings::str_len::StrLenParser::new())),
        //
        // sets
        //
        ("SADD", Box::new(sets::set_add::SetAddParser::new())),
        ("SCARD", Box::new(sets::set_card::SetCardParser::new())),
        ("SDIFF", Box::new(sets::set_diff::SetDiffParser::new())),
        (
            "SDIFFSTORE",
            Box::new(sets::set_diff_store::SetDiffStoreParser::new()),
        ),
        (
            "SINTER",
            Box::new(sets::set_intersect::SetIntersectParser::new()),
        ),
        (
            "SINTERCARD",
            Box::new(sets::set_intersect_card::SetIntersectCardParser::new()),
        ),
        (
            "SINTERSTORE",
            Box::new(sets::set_intersect_store::SetIntersectStoreParser::new()),
        ),
        (
            "SISMEMBER",
            Box::new(sets::set_is_member::SetIsMemberParser::new()),
        ),
        (
            "SMEMBERS",
            Box::new(sets::set_members::SetMembersParser::new()),
        ),
        (
            "SMISMEMBER",
            Box::new(sets::set_is_member_multiple::SetIsMemberMultipleParser::new()),
        ),
        ("SMOVE", Box::new(sets::set_move::SetMoveParser::new())),
        ("SPOP", Box::new(sets::set_pop::SetPopParser::new())),
        (
            "SRANDMEMBER",
            Box::new(sets::set_random_members::SetRandomMembersParser::new()),
        ),
        ("SREM", Box::new(sets::set_remove::SetRemoveParser::new())),
        ("SUNION", Box::new(sets::set_union::SetUnionParser::new())),
        (
            "SUNIONSTORE",
            Box::new(sets::set_union_store::SetUnionStoreParser::new()),
        ),
    ]
}

pub struct CommandTree {
    parser: Option<Box<dyn TryParse>>,
    children: HashMap<char, CommandTree>,
}

impl CommandTree {
    pub fn new() -> Self {
        let mut tree = Self::new_node();
        for (name, parser) in get_commands() {
            tree.insert(name, parser);
        }

        tree
    }

    fn new_node() -> Self {
        Self {
            parser: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, command: &str, parser: Box<dyn TryParse>) {
        let mut current = self;

        for c in command.chars() {
            current = current
                .children
                .entry(c.to_ascii_uppercase())
                .or_insert(Self::new_node());
        }

        current.parser = Some(parser);
    }

    pub fn get(&self, command: &str) -> Option<&dyn TryParse> {
        let mut current = self;

        for c in command.chars() {
            if let Some(next) = current.children.get(&(c.to_ascii_uppercase())) {
                current = next;
            } else {
                return None;
            }
        }

        current.parser.as_deref()
    }
}
