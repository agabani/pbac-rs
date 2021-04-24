#[derive(Debug, PartialEq)]
pub struct ScopedResource {
    pub scope: String,
    pub resource: String,
}

impl ScopedResource {
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(index) => {
                let scope = &value[0..index];
                let resource = &value[index + 1..value.len()];

                Self {
                    scope: scope.to_string(),
                    resource: resource.to_string(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let expected = ScopedResource {
            scope: "scope".to_string(),
            resource: "resource".to_string(),
        };

        let actual = ScopedResource::parse("scope:resource");

        assert_eq!(actual, expected);
    }
}
