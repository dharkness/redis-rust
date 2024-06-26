use std::collections::HashMap;

use crate::parse::TryParse;

mod prelude {
    pub use crate::network::*;
    pub use crate::parse::{Apply, Input, Options, parse_options, TryParse};
    pub use crate::storage::{clamp, clamp_range, IfKindResult, Kind, Store, Value};
}

mod common;
mod expiration;
mod lists;
mod server;
mod sets;
mod strings;

pub struct CommandTree {
    parser: Option<Box<dyn TryParse>>,
    children: HashMap<char, CommandTree>,
}

impl CommandTree {
    pub fn new() -> Self {
        Self::new_node()
            //
            // server
            //
            .insert("COMMAND", Box::new(server::command::CommandParser::new()))
            .insert("ECHO", Box::new(server::echo::EchoParser::new()))
            //
            // common
            //
            .insert("COPY", Box::new(common::copy::CopyParser::new()))
            .insert("DEL", Box::new(common::del::DelParser::new()))
            .insert("EXISTS", Box::new(common::exists::ExistsParser::new()))
            .insert("KEYS", Box::new(common::keys::KeysParser::new()))
            .insert("RENAME", Box::new(common::rename::RenameParser::new()))
            .insert("TYPE", Box::new(common::r#type::TypeParser::new()))
            //
            // expiration
            //
            .insert(
                "EXPIRE",
                Box::new(expiration::expire_s::ExpireSecsParser::new()),
            )
            .insert(
                "EXPIREAT",
                Box::new(expiration::expire_at_s::ExpireAtSecsParser::new()),
            )
            .insert(
                "EXPIRETIME",
                Box::new(expiration::expire_time_s::ExpireTimeSecsParser::new()),
            )
            .insert(
                "PERSIST",
                Box::new(expiration::persist::PersistParser::new()),
            )
            .insert(
                "PEXPIRE",
                Box::new(expiration::expire_ms::ExpireMillisParser::new()),
            )
            .insert(
                "ExpireAtMillis",
                Box::new(expiration::expire_at_ms::ExpireAtMillisParser::new()),
            )
            .insert(
                "PEXPIRETIME",
                Box::new(expiration::expire_time_ms::ExpireTimeMillisParser::new()),
            )
            .insert(
                "PTTL",
                Box::new(expiration::ttl_ms::PTimeToLiveParser::new()),
            )
            .insert("TTL", Box::new(expiration::ttl_s::TimeToLiveParser::new()))
            //
            // lists
            //
            .insert("LINDEX", Box::new(lists::index::IndexParser::new()))
            .insert("LINSERT", Box::new(lists::insert::InsertParser::new()))
            .insert("LLEN", Box::new(lists::len::LenParser::new()))
            .insert("LMOVE", Box::new(lists::r#move::MoveParser::new()))
            .insert(
                "LMPOP",
                Box::new(lists::pop_multiple::PopMultipleParser::new()),
            )
            .insert("LPOP", Box::new(lists::left_pop::LeftPopParser::new()))
            .insert("LPOS", Box::new(lists::position::PositionParser::new()))
            .insert("LPUSH", Box::new(lists::left_push::LeftPushParser::new()))
            .insert(
                "LPUSHX",
                Box::new(lists::left_push_exists::LeftPushExistsParser::new()),
            )
            .insert("LRANGE", Box::new(lists::range::RangeParser::new()))
            .insert("LREM", Box::new(lists::remove::RemoveParser::new()))
            .insert("LSET", Box::new(lists::set::SetParser::new()))
            .insert("LTRIM", Box::new(lists::trim::TrimParser::new()))
            .insert("RPOP", Box::new(lists::right_pop::RightPopParser::new()))
            .insert("RPUSH", Box::new(lists::right_push::RightPushParser::new()))
            .insert(
                "RPUSHX",
                Box::new(lists::right_push_exists::RightPushExistsParser::new()),
            )
            //
            // sets
            //
            .insert("SADD", Box::new(sets::add::AddParser::new()))
            .insert("SCARD", Box::new(sets::card::CardParser::new()))
            .insert("SDIFF", Box::new(sets::diff::DiffParser::new()))
            .insert(
                "SDIFFSTORE",
                Box::new(sets::diff_store::DiffStoreParser::new()),
            )
            .insert("SINTER", Box::new(sets::intersect::IntersectParser::new()))
            .insert(
                "SINTERCARD",
                Box::new(sets::intersect_card::IntersectCardParser::new()),
            )
            .insert(
                "SINTERSTORE",
                Box::new(sets::intersect_store::IntersectStoreParser::new()),
            )
            .insert(
                "SISMEMBER",
                Box::new(sets::is_member::IsMemberParser::new()),
            )
            .insert("SMEMBERS", Box::new(sets::members::MembersParser::new()))
            .insert(
                "SMISMEMBER",
                Box::new(sets::is_member_multiple::IsMemberMultipleParser::new()),
            )
            .insert("SMOVE", Box::new(sets::r#move::MoveParser::new()))
            .insert("SPOP", Box::new(sets::pop::PopParser::new()))
            .insert(
                "SRANDMEMBER",
                Box::new(sets::random_members::RandomMembersParser::new()),
            )
            .insert("SREM", Box::new(sets::remove::RemoveParser::new()))
            .insert("SUNION", Box::new(sets::union::UnionParser::new()))
            .insert(
                "SUNIONSTORE",
                Box::new(sets::union_store::UnionStoreParser::new()),
            )
            //
            // strings
            //
            .insert("APPEND", Box::new(strings::append::AppendParser::new()))
            .insert("GET", Box::new(strings::get::GetParser::new()))
            .insert("GETDEL", Box::new(strings::get_del::GetDelParser::new()))
            .insert("GETEX", Box::new(strings::get_ex::GetExParser::new()))
            .insert(
                "GETRANGE",
                Box::new(strings::get_range::GetRangeParser::new()),
            )
            .insert(
                "MSET",
                Box::new(strings::set_multiple::SetMultipleParser::new()),
            )
            .insert(
                "MSETNX",
                Box::new(strings::set_multiple_if_not_set::SetMultipleIfNotSetParser::new()),
            )
            .insert("SET", Box::new(strings::set::SetParser::new()))
            .insert("STRLEN", Box::new(strings::str_len::StrLenParser::new()))
    }

    fn new_node() -> Self {
        Self {
            parser: None,
            children: HashMap::new(),
        }
    }

    fn insert(mut self, command: &str, parser: Box<dyn TryParse>) -> Self {
        let mut current = &mut self;

        for c in command.chars() {
            current = current
                .children
                .entry(c.to_ascii_uppercase())
                .or_insert(Self::new_node());
        }

        current.parser = Some(parser);
        self
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
