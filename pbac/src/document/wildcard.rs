use crate::Element;

#[derive(Debug, PartialEq)]
pub enum WildcardToken<T> {
    Wildcard,
    Value(T),
}

impl Element<String> for WildcardToken<String> {
    fn is_match(&self, value: &String) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document == value,
        }
    }

    fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(value.to_string()),
        }
    }
}
