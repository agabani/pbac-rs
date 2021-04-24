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
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(index) => {
                let scope = &value[0..index];
                let action = &value[index + 1..value.len()];

                Self {
                    scope: scope.to_string(),
                    action: Action::parse(action),
                }
            }
        }
    }
}

impl Action {
    pub fn parse(value: &str) -> Self {
        match value.find(':') {
            None => panic!("TODO: return error on formatting error"),
            Some(index) => {
                let verb = &value[0..index];
                let resource = &value[index + 1..value.len()];

                Self {
                    verb: verb.to_string(),
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
        let expected = ScopedAction {
            scope: "scope".to_string(),
            action: Action {
                verb: "verb".to_string(),
                resource: "resource".to_string(),
            },
        };

        let actual = ScopedAction::parse("scope:verb:resource");

        assert_eq!(actual, expected);
    }
}
