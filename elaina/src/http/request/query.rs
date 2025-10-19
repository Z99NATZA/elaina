use std::collections::HashMap;

#[derive(Debug)]
pub struct Query {
    query_string: HashMap<String, String>,
}

impl From<&str> for Query {
    fn from(value: &str) -> Self {
        let mut query_string = HashMap::new();

        for character in value.split("&") {
            if let Some(value) = character.find("=") {
                let key = &character[..value];
                let value = &character[value+1..];
                query_string.insert(key.to_string(), value.to_string());
            }
        }

        Query { query_string }
    }
}

impl Query {
    pub fn get(&self, key: &String) -> Option<&String> {
        self.query_string.get(key)
    }
}