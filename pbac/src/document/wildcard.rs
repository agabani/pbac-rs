use crate::document::ElementParseError;
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

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        match value {
            "*" => Ok(Self::Wildcard),
            value => match value.len() {
                0 => Err(ElementParseError {
                    token: value.to_string(),
                }),
                _ => Ok(Self::Value(value.to_string())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = WildcardToken::<String>::Wildcard;

                let result = WildcardToken::<String>::parse("*").unwrap();

                assert_eq!(result, expected);
            }
        }

        mod value {
            use super::*;

            #[test]
            fn pass() {
                let expected = WildcardToken::<String>::Value("value".to_string());

                let result = WildcardToken::<String>::parse("value").unwrap();

                assert_eq!(result, expected);
            }

            #[test]
            fn fail_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let result = WildcardToken::<String>::parse("").unwrap_err();

                assert_eq!(result, expected);
            }
        }
    }

    mod is_match {
        use super::*;

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = WildcardToken::<String>::Wildcard;

                let result = document.is_match(&"value".to_string());

                assert_eq!(result, true);
            }
        }

        mod value {
            use super::*;

            #[test]
            fn pass() {
                let document = WildcardToken::<String>::Value("value".to_string());

                let result = document.is_match(&"value".to_string());

                assert_eq!(result, true);
            }

            #[test]
            fn fail() {
                let document = WildcardToken::<String>::Value("value".to_string());

                let result = document.is_match(&"x".to_string());

                assert_eq!(result, false);
            }
        }
    }
}
