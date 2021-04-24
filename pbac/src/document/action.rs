use crate::action::Action;
use crate::document::wildcard::WildcardToken;
use crate::ScopedAction;

#[derive(Debug, PartialEq)]
pub struct ActionDocument {
    scoped_action: WildcardToken<ScopedActionToken>,
}

impl ActionDocument {
    pub fn parse(value: &str) -> Self {
        Self {
            scoped_action: WildcardToken::<ScopedActionToken>::parse(value),
        }
    }

    pub fn is_match(&self, scoped_action: &ScopedAction) -> bool {
        self.scoped_action.is_match(scoped_action)
    }
}

#[derive(Debug, PartialEq)]
struct ScopedActionToken {
    scope: WildcardToken<String>,
    action: WildcardToken<ActionToken>,
}

impl ScopedActionToken {
    fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let scope = &value[0..position];
                let action = &value[position + 1..value.len()];

                Self {
                    scope: WildcardToken::<String>::parse(scope),
                    action: WildcardToken::<ActionToken>::parse(action),
                }
            }
        }
    }

    fn is_match(&self, scoped_action: &ScopedAction) -> bool {
        self.scope.is_match(&scoped_action.scope) && self.action.is_match(&scoped_action.action)
    }
}

impl WildcardToken<ScopedActionToken> {
    fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(ScopedActionToken::parse(value)),
        }
    }

    fn is_match(&self, scoped_action: &ScopedAction) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document.is_match(scoped_action),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ActionToken {
    verb: WildcardToken<String>,
    resource: WildcardToken<String>,
}

impl ActionToken {
    fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let verb = &value[0..position];
                let resource = &value[position + 1..value.len()];

                Self {
                    verb: WildcardToken::<String>::parse(verb),
                    resource: WildcardToken::<String>::parse(resource),
                }
            }
        }
    }

    fn is_match(&self, action: &Action) -> bool {
        self.verb.is_match(&action.verb) && self.resource.is_match(&action.resource)
    }
}

impl WildcardToken<ActionToken> {
    fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(ActionToken::parse(value)),
        }
    }

    fn is_match(&self, action: &Action) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document.is_match(action),
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

    mod parse {
        use super::*;

        #[test]
        fn wildcard() {
            let expected = WildcardToken::<ScopedActionToken>::Wildcard;

            let actual = WildcardToken::<ScopedActionToken>::parse("*");

            assert_eq!(actual, expected);
        }

        #[test]
        fn scope_verb_resource() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Value("scope".to_string()),
                action: WildcardToken::Value(ActionToken {
                    verb: WildcardToken::Value("verb".to_string()),
                    resource: WildcardToken::Value("resource".to_string()),
                }),
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("scope:verb:resource");

            assert_eq!(actual, expected)
        }

        #[test]
        fn scope_verb_wildcard() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Value("scope".to_string()),
                action: WildcardToken::Value(ActionToken {
                    verb: WildcardToken::Value("verb".to_string()),
                    resource: WildcardToken::Wildcard,
                }),
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("scope:verb:*");

            assert_eq!(actual, expected)
        }

        #[test]
        fn scope_wildcard_resource() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Value("scope".to_string()),
                action: WildcardToken::Value(ActionToken {
                    verb: WildcardToken::Wildcard,
                    resource: WildcardToken::Value("resource".to_string()),
                }),
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("scope:*:resource");

            assert_eq!(actual, expected)
        }

        #[test]
        fn scope_wildcard() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Value("scope".to_string()),
                action: WildcardToken::Wildcard,
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("scope:*");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_verb_resource() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Wildcard,
                action: WildcardToken::Value(ActionToken {
                    verb: WildcardToken::Value("verb".to_string()),
                    resource: WildcardToken::Value("resource".to_string()),
                }),
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("*:verb:resource");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_verb_wildcard() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Wildcard,
                action: WildcardToken::Value(ActionToken {
                    verb: WildcardToken::Value("verb".to_string()),
                    resource: WildcardToken::Wildcard,
                }),
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("*:verb:*");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_wildcard_resource() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Wildcard,
                action: WildcardToken::Value(ActionToken {
                    verb: WildcardToken::Wildcard,
                    resource: WildcardToken::Value("resource".to_string()),
                }),
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("*:*:resource");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_wildcard() {
            let expected = WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                scope: WildcardToken::Wildcard,
                action: WildcardToken::Wildcard,
            });

            let actual = WildcardToken::<ScopedActionToken>::parse("*:*");

            assert_eq!(actual, expected)
        }
    }

    mod is_match {
        use super::*;

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = WildcardToken::<ScopedActionToken>::Wildcard;

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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Value("scope".to_string()),
                    action: WildcardToken::Value(ActionToken {
                        verb: WildcardToken::Value("verb".to_string()),
                        resource: WildcardToken::Value("resource".to_string()),
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Value("scope".to_string()),
                    action: WildcardToken::Value(ActionToken {
                        verb: WildcardToken::Value("verb".to_string()),
                        resource: WildcardToken::Wildcard,
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Value("scope".to_string()),
                    action: WildcardToken::Value(ActionToken {
                        verb: WildcardToken::Wildcard,
                        resource: WildcardToken::Value("resource".to_string()),
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Value("scope".to_string()),
                    action: WildcardToken::Wildcard,
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Wildcard,
                    action: WildcardToken::Value(ActionToken {
                        verb: WildcardToken::Value("verb".to_string()),
                        resource: WildcardToken::Value("resource".to_string()),
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Wildcard,
                    action: WildcardToken::Value(ActionToken {
                        verb: WildcardToken::Value("verb".to_string()),
                        resource: WildcardToken::Wildcard,
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Wildcard,
                    action: WildcardToken::Value(ActionToken {
                        verb: WildcardToken::Wildcard,
                        resource: WildcardToken::Value("resource".to_string()),
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

            fn document() -> WildcardToken<ScopedActionToken> {
                WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                    scope: WildcardToken::Wildcard,
                    action: WildcardToken::Wildcard,
                })
            }
        }
    }
}
