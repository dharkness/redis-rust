use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    List,
    Integer,
    Set,
    String,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::List => "list",
            Kind::Integer => "integer",
            Kind::Set => "set",
            Kind::String => "string",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Integer(i64),
    List(Vec<String>),
    Set(HashSet<String>),
    String(String),
}

impl Value {
    pub fn set_from_vec(members: &[String]) -> Self {
        Value::Set(members.iter().cloned().collect())
    }

    pub fn kind(&self) -> Kind {
        match self {
            Value::List(_) => Kind::List,
            Value::Integer(_) => Kind::Integer,
            Value::Set(_) => Kind::Set,
            Value::String(_) => Kind::String,
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

impl From<Vec<String>> for Value {
    fn from(elements: Vec<String>) -> Self {
        Value::List(elements)
    }
}

impl From<usize> for Value {
    fn from(n: usize) -> Self {
        Value::Integer(n as i64)
    }
}

impl From<HashSet<String>> for Value {
    fn from(members: HashSet<String>) -> Self {
        Value::Set(members)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}
