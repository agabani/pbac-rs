#[derive(Debug, PartialEq)]
pub struct ScopedActionDocument {
    scope: DocumentToken<String>,
    action: DocumentToken<ActionDocument>,
}

#[derive(Debug, PartialEq)]
pub struct ActionDocument {
    verb: DocumentToken<String>,
    resource: DocumentToken<String>,
}

#[derive(Debug, PartialEq)]
pub enum DocumentToken<T> {
    Wildcard,
    Value(T),
}

impl DocumentToken<ScopedActionDocument> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(ScopedActionDocument::parse(value)),
        }
    }
}

impl DocumentToken<ActionDocument> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(ActionDocument::parse(value)),
        }
    }
}

impl DocumentToken<String> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(value.to_string()),
        }
    }
}

impl ScopedActionDocument {
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let scope = &value[0..position];
                let action = &value[position + 1..value.len()];

                Self {
                    scope: DocumentToken::<String>::parse(scope),
                    action: DocumentToken::<ActionDocument>::parse(action),
                }
            }
        }
    }
}

impl ActionDocument {
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let verb = &value[0..position];
                let resource = &value[position + 1..value.len()];

                Self {
                    verb: DocumentToken::<String>::parse(verb),
                    resource: DocumentToken::<String>::parse(resource),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /* All possible combinations:
     *
     *      *
     *
     *      scope:verb:resource
     *      scope:verb:*
     *      scope:*:resource
     *      scope:*
     *
     *      *:verb:resource
     *      *:verb:*
     *      *:*:resource
     *      *:*
     */
    use super::*;

    #[test]
    fn parse_wildcard() {
        let expected = DocumentToken::<ScopedActionDocument>::Wildcard;

        let actual = DocumentToken::<ScopedActionDocument>::parse("*");

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_scope_verb_resource() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Value("scope".to_string()),
            action: DocumentToken::Value(ActionDocument {
                verb: DocumentToken::Value("verb".to_string()),
                resource: DocumentToken::Value("resource".to_string()),
            }),
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("scope:verb:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_scope_verb_wildcard() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Value("scope".to_string()),
            action: DocumentToken::Value(ActionDocument {
                verb: DocumentToken::Value("verb".to_string()),
                resource: DocumentToken::Wildcard,
            }),
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("scope:verb:*");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_scope_wildcard_resource() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Value("scope".to_string()),
            action: DocumentToken::Value(ActionDocument {
                verb: DocumentToken::Wildcard,
                resource: DocumentToken::Value("resource".to_string()),
            }),
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("scope:*:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_scope_wildcard() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Value("scope".to_string()),
            action: DocumentToken::Wildcard,
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("scope:*");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_verb_resource() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Wildcard,
            action: DocumentToken::Value(ActionDocument {
                verb: DocumentToken::Value("verb".to_string()),
                resource: DocumentToken::Value("resource".to_string()),
            }),
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("*:verb:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_verb_wildcard() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Wildcard,
            action: DocumentToken::Value(ActionDocument {
                verb: DocumentToken::Value("verb".to_string()),
                resource: DocumentToken::Wildcard,
            }),
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("*:verb:*");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_wildcard_resource() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Wildcard,
            action: DocumentToken::Value(ActionDocument {
                verb: DocumentToken::Wildcard,
                resource: DocumentToken::Value("resource".to_string()),
            }),
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("*:*:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_wildcard() {
        let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
            scope: DocumentToken::Wildcard,
            action: DocumentToken::Wildcard,
        });

        let actual = DocumentToken::<ScopedActionDocument>::parse("*:*");

        assert_eq!(actual, expected)
    }
}
