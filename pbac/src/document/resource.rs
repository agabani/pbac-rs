use crate::document::wildcard::WildcardToken;
use crate::resource::ScopedResource;
use crate::{Element, ElementParseError};

#[derive(Debug, PartialEq)]
pub struct ResourceDocument {
    scoped_resource: WildcardToken<ScopedResourceToken>,
}

impl Element<ScopedResource> for ResourceDocument {
    fn is_match(&self, value: &ScopedResource) -> bool {
        self.scoped_resource.is_match(value)
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        Ok(Self {
            scoped_resource: WildcardToken::<ScopedResourceToken>::parse(value)?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct ScopedResourceToken {
    scope: WildcardToken<String>,
    resource: WildcardToken<String>,
}

impl Element<ScopedResource> for ScopedResourceToken {
    fn is_match(&self, value: &ScopedResource) -> bool {
        self.scope.is_match(&value.scope) && self.resource.is_match(&value.resource)
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        match value.find(':') {
            None => Err(ElementParseError {
                token: value.to_string(),
            }),
            Some(position) => {
                let scope = &value[0..position];
                let resource = &value[position + 1..value.len()];

                Ok(Self {
                    scope: WildcardToken::<String>::parse(scope)?,
                    resource: WildcardToken::<String>::parse(resource)?,
                })
            }
        }
    }
}

impl Element<ScopedResource> for WildcardToken<ScopedResourceToken> {
    fn is_match(&self, value: &ScopedResource) -> bool {
        match self {
            WildcardToken::Wildcard => true,
            WildcardToken::Value(document) => document.is_match(value),
        }
    }

    fn parse(value: &str) -> Result<Self, ElementParseError> {
        Ok(match value {
            "*" => Self::Wildcard,
            value => Self::Value(ScopedResourceToken::parse(value)?),
        })
    }
}

#[cfg(test)]
mod tests {
    /* All possible combinations:
     *
     *      *
     *
     *      scope:resource
     *      scope:*
     *
     *      *:resource
     *      *:*
     */
    use super::*;

    mod parse {
        use super::*;

        mod wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ResourceDocument {
                    scoped_resource: WildcardToken::Wildcard,
                };

                let actual = ResourceDocument::parse("*").unwrap();

                assert_eq!(actual, expected);
            }

            #[test]
            fn fail_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ResourceDocument::parse("").unwrap_err();

                assert_eq!(actual, expected);
            }

            #[test]
            fn fail_token() {
                let expected = ElementParseError {
                    token: "token".to_string(),
                };

                let actual = ResourceDocument::parse("token").unwrap_err();

                assert_eq!(actual, expected);
            }
        }

        mod scope_resource {
            use super::*;

            #[test]
            fn pass() {
                let expected = ResourceDocument {
                    scoped_resource: WildcardToken::<ScopedResourceToken>::Value(
                        ScopedResourceToken {
                            scope: WildcardToken::Value("scope".to_string()),
                            resource: WildcardToken::Value("resource".to_string()),
                        },
                    ),
                };

                let actual = ResourceDocument::parse("scope:resource").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_scope_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ResourceDocument::parse("scope:").unwrap_err();

                assert_eq!(actual, expected);
            }

            #[test]
            fn fail_empty_resource() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ResourceDocument::parse(":resource").unwrap_err();

                assert_eq!(actual, expected);
            }

            #[test]
            fn fail_empty_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ResourceDocument::parse(":").unwrap_err();

                assert_eq!(actual, expected);
            }
        }

        mod scope_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ResourceDocument {
                    scoped_resource: WildcardToken::<ScopedResourceToken>::Value(
                        ScopedResourceToken {
                            scope: WildcardToken::Value("scope".to_string()),
                            resource: WildcardToken::Wildcard,
                        },
                    ),
                };

                let actual = ResourceDocument::parse("scope:*").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_empty_wildcard() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ResourceDocument::parse(":*").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod wildcard_resource {
            use super::*;

            #[test]
            fn pass() {
                let expected = ResourceDocument {
                    scoped_resource: WildcardToken::<ScopedResourceToken>::Value(
                        ScopedResourceToken {
                            scope: WildcardToken::Wildcard,
                            resource: WildcardToken::Value("resource".to_string()),
                        },
                    ),
                };

                let actual = ResourceDocument::parse("*:resource").unwrap();

                assert_eq!(actual, expected)
            }

            #[test]
            fn fail_wildcard_empty() {
                let expected = ElementParseError {
                    token: "".to_string(),
                };

                let actual = ResourceDocument::parse("*:").unwrap_err();

                assert_eq!(actual, expected)
            }
        }

        mod wildcard_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let expected = ResourceDocument {
                    scoped_resource: WildcardToken::<ScopedResourceToken>::Value(
                        ScopedResourceToken {
                            scope: WildcardToken::Wildcard,
                            resource: WildcardToken::Wildcard,
                        },
                    ),
                };

                let actual = ResourceDocument::parse("*:*").unwrap();

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
                let document = ResourceDocument {
                    scoped_resource: WildcardToken::Wildcard,
                };

                let scoped_action = ScopedResource::parse("scope:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }
        }

        mod scope_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> ResourceDocument {
                ResourceDocument {
                    scoped_resource: WildcardToken::Value(ScopedResourceToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        resource: WildcardToken::Value("resource".to_string()),
                    }),
                }
            }
        }

        mod scope_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            fn document() -> ResourceDocument {
                ResourceDocument {
                    scoped_resource: WildcardToken::Value(ScopedResourceToken {
                        scope: WildcardToken::Value("scope".to_string()),
                        resource: WildcardToken::Wildcard,
                    }),
                }
            }
        }

        mod wildcard_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            fn document() -> ResourceDocument {
                ResourceDocument {
                    scoped_resource: WildcardToken::Value(ScopedResourceToken {
                        scope: WildcardToken::Wildcard,
                        resource: WildcardToken::Value("resource".to_string()),
                    }),
                }
            }
        }

        mod wildcard_wildcard {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x").unwrap();

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            fn document() -> ResourceDocument {
                ResourceDocument {
                    scoped_resource: WildcardToken::Value(ScopedResourceToken {
                        scope: WildcardToken::Wildcard,
                        resource: WildcardToken::Wildcard,
                    }),
                }
            }
        }
    }
}
