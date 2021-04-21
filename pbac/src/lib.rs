#[derive(Debug, PartialEq)]
pub struct Action(String);

#[derive(Debug, PartialEq)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, PartialEq)]
pub struct Principal(String);

#[derive(Debug, PartialEq)]
pub struct Resource(String);

#[derive(Debug, PartialEq)]
pub struct Policy {
    pub actions: Vec<Action>,
    pub effect: Effect,
    pub principals: Vec<Principal>,
    pub resources: Vec<Resource>,
}

pub fn evaluate() -> (Effect, Option<Policy>) {
    (Effect::Deny, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_denies_by_default() {
        let (effect, policy) = evaluate();

        assert_eq!(Effect::Deny, effect);
        assert_eq!(None, policy);
    }
}
