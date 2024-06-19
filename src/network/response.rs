use std::collections::HashSet;

use crate::storage::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response<'a> {
    Null,
    Ok,
    Zero,
    One,
    False,
    True,
    EmptyBulkString,
    EmptyList,
    EmptyMap,
    EmptySet,
    I64(i64),
    Usize(usize),
    Raw(&'static [u8]),
    SimpleString(String),
    BulkString(String),
    List(Vec<String>),
    Set(HashSet<String>),
    Value(Value),
    ValueRef(&'a Value),
    ValueList(Vec<Value>),
}

impl<'a> Response<'a> {
    pub fn int_from_bool(b: bool) -> Response<'a> {
        if b {
            Response::One
        } else {
            Response::Zero
        }
    }
}
