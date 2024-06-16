use std::collections::HashSet;

const SET: &str = "set";
const STRING: &str = "string";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    String,
    Set,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::String => STRING,
            Kind::Set => SET,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    String(String),
    Set(HashSet<String>),
}

impl Value {
    pub fn new_string(s: String) -> Self {
        Value::String(s)
    }

    pub fn new_set(members: &[String]) -> Self {
        Value::Set(members.iter().cloned().collect())
    }

    pub fn kind(&self) -> Kind {
        match self {
            Value::String(_) => Kind::String,
            Value::Set(_) => Kind::Set,
        }
    }

    pub fn expect_set(&self) -> &HashSet<String> {
        match self {
            Value::Set(members) => members,
            _ => panic!("expected set"),
        }
    }

    pub fn expect_set_mut(&mut self) -> &mut HashSet<String> {
        match self {
            Value::Set(members) => members,
            _ => panic!("expected set"),
        }
    }
}
