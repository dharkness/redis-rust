const STRING: &str = "string";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    String,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::String => STRING,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    String(String),
}

impl Value {
    pub fn new_string(s: String) -> Self {
        Value::String(s)
    }

    pub fn kind(&self) -> Kind {
        match self {
            Value::String(_) => Kind::String,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::String(s) => s,
        }
    }
}
