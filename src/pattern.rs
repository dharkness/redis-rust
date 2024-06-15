use regex::Regex;

/// Pattern uses a regular expression to match keys
/// instead of Redis' pattern matching.
pub struct Pattern {
    pattern: Regex,
}

impl Pattern {
    pub fn try_parse(pattern: &str) -> Result<Self, String> {
        Ok(Self {
            pattern: parse_pattern(pattern)?,
        })
    }

    pub fn matches(&self, key: &String) -> bool {
        self.pattern.is_match(key)
    }
}

fn parse_pattern(str: &str) -> Result<Regex, String> {
    let mut pattern = String::new();

    pattern.push_str(str);

    Regex::new(&pattern).map_err(|err| err.to_string())
}
