use crate::action::Action;
use crate::document::wildcard::WildcardToken;
use crate::document::Element;
use crate::{ElementParseError, ScopedAction};

#[derive(Debug, PartialEq)]
pub struct ActionDocument {
    scoped_action: WildcardToken<ScopedActionToken>,
}

impl Element<ScopedAction> for ActionDocument {
    fn is_match(&self, value: &ScopedAction) -> bool {
        self.scoped_action.is_match(value)
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        Ok(Self {
            scoped_action: WildcardToken::<ScopedActionToken>::parse(value)?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct ScopedActionToken {
    scope: WildcardToken<String>,
    action: WildcardToken<ActionToken>,
}

impl Element<ScopedAction> for ScopedActionToken {
    fn is_match(&self, value: &ScopedAction) -> bool {
        self.scope.is_match(&value.scope) && self.action.is_match(&value.action)
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        match value.find(':') {
            None => Err(ElementParseError {
                token: value.to_string(),
            }),
            Some(position) => {
                let scope = &value[0..position];
                let action = &value[position + 1..value.len()];

                Ok(Self {
                    scope: WildcardToken::<String>::parse(scope)?,
                    action: WildcardToken::<ActionToken>::parse(action)?,
                })
            }
        }
    }
}
impl Element<ScopedAction> for WildcardToken<ScopedActionToken> {
    fn is_match(&self, value: &ScopedAction) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document.is_match(value),
        }
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        Ok(match value {
            "*" => Self::Wildcard,
            value => Self::Value(ScopedActionToken::parse(value)?),
        })
    }
}

#[derive(Debug, PartialEq)]
struct ActionToken {
    verb: WildcardToken<String>,
    resource: WildcardToken<String>,
}

impl Element<Action> for ActionToken {
    fn is_match(&self, value: &Action) -> bool {
        self.verb.is_match(&value.verb) && self.resource.is_match(&value.resource)
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        match value.find(':') {
            None => Err(ElementParseError {
                token: value.to_string(),
            }),
            Some(position) => {
                let verb = &value[0..position];
                let resource = &value[position + 1..value.len()];

                Ok(Self {
                    verb: WildcardToken::<String>::parse(verb)?,
                    resource: WildcardToken::<String>::parse(resource)?,
                })
            }
        }
    }
}

impl Element<Action> for WildcardToken<ActionToken> {
    fn is_match(&self, value: &Action) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document.is_match(value),
        }
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        Ok(match value {
            "*" => Self::Wildcard,
            value => Self::Value(ActionToken::parse(value)?),
        })
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

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Wildcard,
                };

                let actual = ActionDocument::parse("*").unwrap();

                assert_eq!(actual, expected);
            }

            #[test]
            fn fail_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("").unwrap_err();

                assert_eq!(actual, expected);
            }

            #[test]
            fn fail_token() {
                let expected = ElementParseError {
                    token: "token".to_string(),
                };

                let actual = ActionDocument::parse("token").unwrap_err();

                assert_eq!(actual, expected);
            }
        }

        mod scope_verb_resource {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Value("verb".to_string()),
                            resource: WildcardToken::Value("resource".to_string()),
                        }),
                    }),
                };

                let actual = ActionDocument::parse("scope:verb:resource").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_scope_verb_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("scope:verb:").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_scope_empty_resource() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("scope::resource").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_scope_empty_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("scope::").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_verb_resource() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse(":verb:resource").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_verb_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse(":verb:").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_empty_resource() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("::resource").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_empty_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("::").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod scope_verb_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Value("verb".to_string()),
                            resource: WildcardToken::Wildcard,
                        }),
                    }),
                };

                let actual = ActionDocument::parse("scope:verb:*").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_scope_empty_wildcard() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("scope::*").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_verb_wildcard() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse(":verb:*").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_empty_wildcard() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("::*").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod scope_wildcard_resource {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Wildcard,
                            resource: WildcardToken::Value("resource".to_string()),
                        }),
                    }),
                };

                let actual = ActionDocument::parse("scope:*:resource").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_scope_wildcard_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("scope:*:").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_wildcard_resource() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse(":*:resource").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_wildcard_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse(":*:").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod scope_wildcard {
            use super::*;

            #[test]
            fn scope_wildcard() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        action: WildcardToken::Wildcard,
                    }),
                };

                let actual = ActionDocument::parse("scope:*").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_wildcard() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse(":*").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod wildcard_verb_resource {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Wildcard,
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Value("verb".to_string()),
                            resource: WildcardToken::Value("resource".to_string()),
                        }),
                    }),
                };

                let actual = ActionDocument::parse("*:verb:resource").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_verb_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("*:verb:").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_empty_resource() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("*::resource").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_empty_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("*::").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod wildcard_verb_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Wildcard,
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Value("verb".to_string()),
                            resource: WildcardToken::Wildcard,
                        }),
                    }),
                };

                let actual = ActionDocument::parse("*:verb:*").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_empty_wildcard() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("*::*").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod wildcard_wildcard_resource {
            use super::*;

            #[test]
            fn wildcard_wildcard_resource() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Wildcard,
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Wildcard,
                            resource: WildcardToken::Value("resource".to_string()),
                        }),
                    }),
                };

                let actual = ActionDocument::parse("*:*:resource").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_wildcard_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("*:*:").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod wildcard_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Wildcard,
                        action: WildcardToken::Wildcard,
                    }),
                };

                let actual = ActionDocument::parse("*:*").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ActionDocument::parse("*:").unwrap_err();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_token() {
                let expected = ElementParseError {
                    token: "token".to_string(),
                };

                let actual = ActionDocument::parse("*:token").unwrap_err();

                assert_eq!(actual, expected)
            }
        }
    }

    mod is_match {
        use super::*;

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Wildcard,
                };

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }
        }

        mod scope_verb_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> ActionDocument {
                ActionDocument {
                    scoped_action: WildcardToken::<ScopedActionToken>::Value(ScopedActionToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        action: WildcardToken::Value(ActionToken {
                            verb: WildcardToken::Value("verb".to_string()),
                            resource: WildcardToken::Value("resource".to_string()),
                        }),
                    }),
                }
            }
        }

        mod scope_verb_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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

                let scoped_action = ScopedAction::parse("scope:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedAction::parse("x:verb:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_verb() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedAction::parse("scope:verb:x").unwrap();

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
