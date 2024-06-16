use super::prelude::*;

const EMPTY: &[u8] = b"$0\r\n\r\n";

struct GetRange {
    key: String,
    start: i64,
    end: i64,
}

impl GetRange {
    pub fn new(key: String, start: i64, end: i64) -> Self {
        Self { key, start, end }
    }
}

impl Apply for GetRange {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        if let Some(value) = store.get(&self.key) {
            match value {
                Value::String(s) => {
                    let len = s.len() as i64;
                    let mut start = if self.start < 0 {
                        len + self.start
                    } else {
                        self.start
                    };
                    let mut end = 1 + if self.end < 0 {
                        len + self.end
                    } else {
                        self.end
                    };

                    if start < 0 {
                        start = 0;
                    } else if start > len {
                        start = len;
                    }
                    if end < 0 {
                        end = 0;
                    } else if end > len {
                        end = len;
                    }

                    if end > start {
                        client.write_bulk_string(&s[start as usize..end as usize], registry)
                    } else {
                        client.write(EMPTY, registry)
                    }
                }
                _ => client.write_simple_error(WRONG_TYPE, registry),
            }
        } else {
            client.write(EMPTY, registry)
        }
    }
}

pub struct GetRangeParser {}

impl GetRangeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetRangeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, String> {
        let key = input.next_string()?;
        let start = input
            .next()?
            .parse::<i64>()
            .map_err(|_| "invalid start".to_string())?;
        let end = input
            .next()?
            .parse::<i64>()
            .map_err(|_| "invalid end".to_string())?;

        Ok(Box::new(GetRange::new(key, start, end)))
    }
}
