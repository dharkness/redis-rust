use super::prelude::*;

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

impl Command for GetRange {
    fn apply(&self, store: &mut Store, client: &mut Client, registry: &Registry) -> io::Result<()> {
        let substring = if let Some(value) = store.get(&self.key) {
            let len = value.len() as i64;
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
                value[start as usize..end as usize].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        client.write_bulk_string(&substring, registry)
    }
}

pub struct GetRangeParser {}

impl GetRangeParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetRangeParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Command>, String> {
        let key = input.next()?;
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
