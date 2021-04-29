use crate::ElementParseError;

#[derive(Debug, PartialEq)]
pub struct ScopedAction {
    pub scope: String,
    pub action: Action,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    pub verb: String,
    pub resource: String,
}

impl ScopedAction {
    pub fn parse(value: &str) -> Result<Self, ElementParseError> {
        match value.find(':') {
            None => Err(ElementParseError {
                token: value.to_string(),
            }),
            Some(index) => {
                let scope = &value[0..index];

                if scope.is_empty() {
                    return Err(ElementParseError {
                        token: scope.to_string(),
                    });
                }

                let action = &value[index + 1..value.len()];

                Ok(Self {
                    scope: scope.to_string(),
                    action: Action::parse(action)?,
                })
            }
        }
    }
}

impl Action {
    pub fn parse(value: &str) -> Result<Self, ElementParseError> {
        match value.find(':') {
            None => Err(ElementParseError {
                token: value.to_string(),
            }),
            Some(index) => {
                let verb = &value[0..index];

                if verb.is_empty() {
                    return Err(ElementParseError {
                        token: verb.to_string(),
                    });
                }

                let resource = &value[index + 1..value.len()];

                if resource.is_empty() {
                    return Err(ElementParseError {
                        token: resource.to_string(),
                    });
                }

                Ok(Self {
                    verb: verb.to_string(),
                    resource: resource.to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn pass() {
            let expected = ScopedAction {
                scope: "scope".to_string(),
                action: Action {
                    verb: "verb".to_string(),
                    resource: "resource".to_string(),
                },
            };

            let actual = ScopedAction::parse("scope:verb:resource").unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_scope_verb_empty() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse("scope:verb:").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_scope_empty_resource() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse("scope::resource").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_scope_empty_empty() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse("scope::").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_scope_token() {
            let expected = ElementParseError {
                token: "token".to_string(),
            };

            let actual = ScopedAction::parse("scope:token").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_verb_resource() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse(":verb:resource").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_verb_empty() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse(":verb:").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_empty_resource() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse("::resource").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_empty_empty() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse("::").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_token() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedAction::parse(":token").unwrap_err();

            assert_eq!(actual, expected);
        }
    }
}
