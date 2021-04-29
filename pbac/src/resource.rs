use crate::ElementParseError;

#[derive(Debug, PartialEq)]
pub struct ScopedResource {
    pub scope: String,
    pub resource: String,
}

impl ScopedResource {
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

                let resource = &value[index + 1..value.len()];

                if resource.is_empty() {
                    return Err(ElementParseError {
                        token: resource.to_string(),
                    });
                }

                Ok(Self {
                    scope: scope.to_string(),
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
            let expected = ScopedResource {
                scope: "scope".to_string(),
                resource: "resource".to_string(),
            };

            let actual = ScopedResource::parse("scope:resource").unwrap();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_scope_empty() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedResource::parse("scope:").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_resource() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedResource::parse(":resource").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_empty_empty() {
            let expected = ElementParseError {
                token: "".to_string(),
            };

            let actual = ScopedResource::parse(":").unwrap_err();

            assert_eq!(actual, expected);
        }

        #[test]
        fn fail_token() {
            let expected = ElementParseError {
                token: "token".to_string(),
            };

            let actual = ScopedResource::parse("token").unwrap_err();

            assert_eq!(actual, expected);
        }
    }
}
