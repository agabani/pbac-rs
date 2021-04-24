#[derive(Debug, PartialEq)]
pub enum WildcardToken<T> {
    Wildcard,
    Value(T),
}

impl WildcardToken<String> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(value.to_string()),
        }
    }

    pub fn is_match(&self, value: &str) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document == value,
        }
    }
}
