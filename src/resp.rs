#[derive(Clone, Debug)]
pub enum Value {
    // Null,
    // Boolean(bool),
    // Integer(i64),
    // BigInteger(String),
    // Double(f64),
    String(String),
    // Error(String),
    Array(Vec<Value>),
    // Set(Vec<Value>),
    // Map(Vec<(Value, Value)>),
}

impl Value {
    pub fn command(&self) -> Result<Command, String> {
        match self {
            Value::Array(values) => {
                if values.is_empty() {
                    return Err("empty array".to_string());
                }
                let command = match &values[0] {
                    Value::String(s) => s,
                    _ => return Err("expected string".to_string()),
                };
                match command.to_uppercase().as_str() {
                    "PING" => Ok(Command::Ping),
                    "ECHO" => {
                        if values.len() != 2 {
                            return Err("expected 1 argument for ECHO".to_string());
                        }
                        match &values[1] {
                            Value::String(s) => Ok(Command::Echo(s.clone())),
                            _ => Err("expected string for arg 1 of ECHO".to_string()),
                        }
                    }
                    "SET" => {
                        if values.len() != 3 {
                            return Err("expected 2 arguments for SET".to_string());
                        }
                        match values[1].clone() {
                            Value::String(key) => {
                                match values[2].clone() {
                                    Value::String(value) => {
                                        return Ok(Command::Set(key, value));
                                    }
                                    _ => Err("expected string for arg 1 of SET".to_string()),
                                }
                            }
                            _ => Err("expected string for arg 1 of SET".to_string()),
                        }
                    }
                    "GET" => {
                        if values.len() != 2 {
                            return Err("expected 1 argument for GET".to_string());
                        }
                        match values[1].clone() {
                            Value::String(key) => {
                                return Ok(Command::Get(key));
                            }
                            _ => Err("expected string for arg 1 of GET".to_string()),
                        }
                    }
                    _ => Err("unknown command".to_string()),
                }
            }
            _ => Err("expected array".to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, String),
    Get(String),
}

pub struct Parser {
    input: String,
}

impl Parser {
    pub fn new() -> Self {
        Self { input: String::new() }
    }

    pub fn add(&mut self, input: &str)  {
        self.input.push_str(input);
    }

    pub fn try_parse_command(&mut self) -> Result<Option<Command>, String> {
        if self.input.is_empty() {
            return Ok(None);
        }
        if self.input.as_bytes()[0] != b'*' {
            return Err("expected '*'".to_string());
        }
        let mut index = 0;
        if let Some(value) = self.try_parse_array(&mut index)? {
            self.input = self.input.split_off(index);
            return Ok(Some(value.command()?))
        } else {
            return Ok(None)
        }
    }

    fn try_parse_value(&mut self, index: &mut usize) -> Result<Option<Value>, String> {
        if self.input.is_empty() {
            return Ok(None);
        }
        match self.input[*index..].as_bytes()[0] {
            // '+' => self.parse_simple_string(),
            // '-' => self.parse_error(),
            // ':' => self.parse_integer(),
            b'$' => self.try_parse_bulk_string(index),
            b'*' => self.try_parse_array(index),
            _ => Err("unexpected type character".to_string()),
        }
    }

    pub fn try_parse_array(&mut self, index: &mut usize) -> Result<Option<Value>, String> {
        println!("index: {}", index);
        debug_assert!(self.input.as_bytes()[*index] == b'*');
        if let Some(end) = self.input[*index..].find("\r\n") {
            println!("end: {}", end);
            let line = &self.input[*index+1..*index+end];
            println!("line: {}", line);
            let len = line.parse::<usize>().map_err(|_| "invalid array length".to_string())?;
            let mut values = Vec::with_capacity(len);
            *index += end + 2;
            for _ in 0..len {
                if let Some(value) = self.try_parse_value(index)? {
                    values.push(value);
                } else {
                    return Ok(None)
                }
            }
            Ok(Some(Value::Array(values)))
        } else {
            Ok(None)
        }
    }

    fn try_parse_bulk_string(&mut self, index: &mut usize) -> Result<Option<Value>, String> {
        debug_assert!(self.input.as_bytes()[*index] == b'$');
        if let Some(end) = self.input[*index..].find("\r\n") {
            let line = &self.input[*index+1..*index+end];
            let len = line.parse::<usize>().map_err(|_| "invalid bulk string length".to_string())?;
            let start = *index+end+2;
            if len > self.input.len() - (start + 2) {
                return Ok(None)
            }
            *index = start + len + 2;
            Ok(Some(Value::String(self.input[start..start+len].to_string())))
        } else {
            Ok(None)
        }
    }
}
