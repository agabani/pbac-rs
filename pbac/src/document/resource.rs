use crate::document::wildcard::WildcardToken;
use crate::resource::ScopedResource;
use crate::Element;

#[derive(Debug, PartialEq)]
pub struct ResourceDocument {
    scoped_resource: WildcardToken<ScopedResourceToken>,
}

impl Element<ScopedResource> for ResourceDocument {
    fn is_match(&self, value: &ScopedResource) -> bool {
        self.scoped_resource.is_match(value)
    }

    fn parse(value: &str) -> Self {
        Self {
            scoped_resource: WildcardToken::<ScopedResourceToken>::parse(value),
        }
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

    fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(position) => {
                let scope = &value[0..position];
                let resource = &value[position + 1..value.len()];

                Self {
                    scope: WildcardToken::<String>::parse(scope),
                    resource: WildcardToken::<String>::parse(resource),
                }
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

    fn parse(value: &str) -> Self {
        match value {
            "*" => Self::Wildcard,
            value => Self::Value(ScopedResourceToken::parse(value)),
        }
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

        #[test]
        fn wildcard() {
            let expected = ResourceDocument {
                scoped_resource: WildcardToken::Wildcard,
            };

            let actual = ResourceDocument::parse("*");

            assert_eq!(actual, expected);
        }

        #[test]
        fn scope_resource() {
            let expected = ResourceDocument {
                scoped_resource: WildcardToken::<ScopedResourceToken>::Value(ScopedResourceToken {
                    scope: WildcardToken::Value("scope".to_string()),
                    resource: WildcardToken::Value("resource".to_string()),
                }),
            };

            let actual = ResourceDocument::parse("scope:resource");

            assert_eq!(actual, expected)
        }

        #[test]
        fn scope_wildcard() {
            let expected = ResourceDocument {
                scoped_resource: WildcardToken::<ScopedResourceToken>::Value(ScopedResourceToken {
                    scope: WildcardToken::Value("scope".to_string()),
                    resource: WildcardToken::Wildcard,
                }),
            };

            let actual = ResourceDocument::parse("scope:*");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_resource() {
            let expected = ResourceDocument {
                scoped_resource: WildcardToken::<ScopedResourceToken>::Value(ScopedResourceToken {
                    scope: WildcardToken::Wildcard,
                    resource: WildcardToken::Value("resource".to_string()),
                }),
            };

            let actual = ResourceDocument::parse("*:resource");

            assert_eq!(actual, expected)
        }

        #[test]
        fn wildcard_wildcard() {
            let expected = ResourceDocument {
                scoped_resource: WildcardToken::<ScopedResourceToken>::Value(ScopedResourceToken {
                    scope: WildcardToken::Wildcard,
                    resource: WildcardToken::Wildcard,
                }),
            };

            let actual = ResourceDocument::parse("*:*");

            assert_eq!(actual, expected)
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

                let scoped_action = ScopedResource::parse("scope:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }
        }

        mod scope_resource {
            use super::*;

            #[test]
            fn pass() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x");

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

                let scoped_action = ScopedResource::parse("scope:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, false);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x");

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

                let scoped_action = ScopedResource::parse("scope:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn fail_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x");

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

                let scoped_action = ScopedResource::parse("scope:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_scope() {
                let document = document();

                let scoped_action = ScopedResource::parse("x:resource");

                let result = document.is_match(&scoped_action);

                assert_eq!(result, true);
            }

            #[test]
            fn pass_resource() {
                let document = document();

                let scoped_action = ScopedResource::parse("scope:x");

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
