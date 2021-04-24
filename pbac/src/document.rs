use crate::action::{Action, ScopedAction};

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

    pub fn is_match(&self, scoped_action: &ScopedAction) -> bool {
        match self {
            DocumentToken::Wildcard => true,
            DocumentToken::Value(document) => document.is_match(scoped_action),
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

    pub fn is_match(&self, action: &Action) -> bool {
        match self {
            DocumentToken::Wildcard => true,
            DocumentToken::Value(document) => document.is_match(action),
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

    pub fn is_match(&self, value: &str) -> bool {
        match self {
            DocumentToken::Wildcard => true,
            DocumentToken::Value(document) => document == value,
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

    pub fn is_match(&self, scoped_action: &ScopedAction) -> bool {
        self.scope.is_match(&scoped_action.scope) && self.action.is_match(&scoped_action.action)
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

    pub fn is_match(&self, action: &Action) -> bool {
        self.verb.is_match(&action.verb) && self.resource.is_match(&action.resource)
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

    mod parse {
        use super::*;

        #[test]
        fn wildcard() {
            let expected = DocumentToken::<ScopedActionDocument>::Wildcard;

            let actual = DocumentToken::<ScopedActionDocument>::parse("*");

            assert_eq!(actual, expected);
        }

        #[test]
        fn scope_verb_resource() {
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
        fn scope_verb_wildcard() {
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
        fn scope_wildcard_resource() {
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
        fn scope_wildcard() {
            let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                scope: DocumentToken::Value("scope".to_string()),
                action: DocumentToken::Wildcard,
            });

            let actual = DocumentToken::<ScopedActionDocument>::parse("scope:*");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_verb_resource() {
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
        fn wildcard_verb_wildcard() {
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
        fn wildcard_wildcard_resource() {
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
        fn wildcard_wildcard() {
            let expected = DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                scope: DocumentToken::Wildcard,
                action: DocumentToken::Wildcard,
            });

            let actual = DocumentToken::<ScopedActionDocument>::parse("*:*");

            assert_eq!(actual, expected)
        }
    }

    mod is_match {
        use super::*;

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = DocumentToken::<ScopedActionDocument>::Wildcard;

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }
        }

        mod scope_verb_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Value("scope".to_string()),
                    action: DocumentToken::Value(ActionDocument {
                        verb: DocumentToken::Value("verb".to_string()),
                        resource: DocumentToken::Value("resource".to_string()),
                    }),
                })
            }
        }

        mod scope_verb_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Value("scope".to_string()),
                    action: DocumentToken::Value(ActionDocument {
                        verb: DocumentToken::Value("verb".to_string()),
                        resource: DocumentToken::Wildcard,
                    }),
                })
            }
        }

        mod scope_wildcard_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Value("scope".to_string()),
                    action: DocumentToken::Value(ActionDocument {
                        verb: DocumentToken::Wildcard,
                        resource: DocumentToken::Value("resource".to_string()),
                    }),
                })
            }
        }

        mod scope_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Value("scope".to_string()),
                    action: DocumentToken::Wildcard,
                })
            }
        }

        mod wildcard_verb_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Wildcard,
                    action: DocumentToken::Value(ActionDocument {
                        verb: DocumentToken::Value("verb".to_string()),
                        resource: DocumentToken::Value("resource".to_string()),
                    }),
                })
            }
        }

        mod wildcard_verb_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Wildcard,
                    action: DocumentToken::Value(ActionDocument {
                        verb: DocumentToken::Value("verb".to_string()),
                        resource: DocumentToken::Wildcard,
                    }),
                })
            }
        }

        mod wildcard_wildcard_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Wildcard,
                    action: DocumentToken::Value(ActionDocument {
                        verb: DocumentToken::Wildcard,
                        resource: DocumentToken::Value("resource".to_string()),
                    }),
                })
            }
        }

        mod wildcard_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            fn document() -> DocumentToken<ScopedActionDocument> {
                DocumentToken::<ScopedActionDocument>::Value(ScopedActionDocument {
                    scope: DocumentToken::Wildcard,
                    action: DocumentToken::Wildcard,
                })
            }
        }
    }
}
