#[derive(Debug, PartialEq)]
pub enum Token<T> {
    Wildcard,
    Value(T),
}

#[derive(Debug, PartialEq)]
pub struct ScopedAction {
    scope: Token<String>,
    action: Token<Action>,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    verb: Token<String>,
    resource: Token<String>,
}

impl Token<ScopedAction> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(ScopedAction::parse(value)),
        }
    }
}

impl Token<Action> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(Action::parse(value)),
        }
    }
}

impl Token<String> {
    pub fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(value.to_string()),
        }
    }
}

impl ScopedAction {
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let scope = &value[0..position];
                let action = &value[position + 1..value.len()];

                Self {
                    scope: Token::<String>::parse(scope),
                    action: Token::<Action>::parse(action),
                }
            }
        }
    }
}

impl Action {
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let verb = &value[0..position];
                let resource = &value[position + 1..value.len()];

                Self {
                    verb: Token::<String>::parse(verb),
                    resource: Token::<String>::parse(resource),
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
        let expected = Token::<ScopedAction>::Wildcard;

        let actual = Token::<ScopedAction>::parse("*");

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_scope_verb_resource() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Value("scope".to_string()),
            action: Token::Value(Action {
                verb: Token::Value("verb".to_string()),
                resource: Token::Value("resource".to_string()),
            }),
        });

        let actual = Token::<ScopedAction>::parse("scope:verb:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_scope_verb_wildcard() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Value("scope".to_string()),
            action: Token::Value(Action {
                verb: Token::Value("verb".to_string()),
                resource: Token::Wildcard,
            }),
        });

        let actual = Token::<ScopedAction>::parse("scope:verb:*");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_scope_wildcard_resource() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Value("scope".to_string()),
            action: Token::Value(Action {
                verb: Token::Wildcard,
                resource: Token::Value("resource".to_string()),
            }),
        });

        let actual = Token::<ScopedAction>::parse("scope:*:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_scope_wildcard() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Value("scope".to_string()),
            action: Token::Wildcard,
        });

        let actual = Token::<ScopedAction>::parse("scope:*");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_verb_resource() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Wildcard,
            action: Token::Value(Action {
                verb: Token::Value("verb".to_string()),
                resource: Token::Value("resource".to_string()),
            }),
        });

        let actual = Token::<ScopedAction>::parse("*:verb:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_verb_wildcard() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Wildcard,
            action: Token::Value(Action {
                verb: Token::Value("verb".to_string()),
                resource: Token::Wildcard,
            }),
        });

        let actual = Token::<ScopedAction>::parse("*:verb:*");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_wildcard_resource() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Wildcard,
            action: Token::Value(Action {
                verb: Token::Wildcard,
                resource: Token::Value("resource".to_string()),
            }),
        });

        let actual = Token::<ScopedAction>::parse("*:*:resource");

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_wildcard_wildcard() {
        let expected = Token::<ScopedAction>::Value(ScopedAction {
            scope: Token::Wildcard,
            action: Token::Wildcard,
        });

        let actual = Token::<ScopedAction>::parse("*:*");

        assert_eq!(actual, expected)
    }
}
