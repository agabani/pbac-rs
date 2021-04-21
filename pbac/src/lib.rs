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

pub fn evaluate<'a>(
    policies: &'a [Policy],
    action: &Action,
    resources: &[Resource],
) -> (Effect, Vec<&'a Policy>) {
    let allowed = policies
        .iter()
        .filter(|policy| {
            policy.effect == Effect::Allow
                && policy.actions.contains(action)
                && policy
                    .resources
                    .iter()
                    .any(|resource| resources.contains(resource))
        })
        .collect::<Vec<_>>();

    if !allowed.is_empty() {
        return (Effect::Allow, allowed);
    }

    (Effect::Deny, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_denies_by_default() {
        let polices = vec![];
        let action = Action("noop".to_string());
        let resources = vec![];

        let (effect, policy) = evaluate(&polices, &action, &resources);

        assert_eq!(effect, Effect::Deny);
        assert_eq!(policy, Vec::<&Policy>::new());
    }

    #[test]
    fn it_allows_when_policy_match() {
        let policies = vec![Policy {
            actions: vec![Action("account:GetAccount".to_string())],
            effect: Effect::Allow,
            principals: vec![Principal("user:JohnSmith".to_string())],
            resources: vec![Resource("account:JohnSmith".to_string())],
        }];
        let action = Action("account:GetAccount".to_string());
        let resources = vec![Resource("account:JohnSmith".to_string())];

        let (effect, policy) = evaluate(&policies, &action, &resources);

        assert_eq!(effect, Effect::Allow);
        assert_eq!(policy, &[policy[0]]);
    }

    #[test]
    fn it_denies_when_action_does_not_match() {
        let policies = vec![Policy {
            actions: vec![Action("account:GetAccount".to_string())],
            effect: Effect::Allow,
            principals: vec![Principal("user:JohnSmith".to_string())],
            resources: vec![Resource("account:JohnSmith".to_string())],
        }];
        let action = Action("account:UpdateAccount".to_string());
        let resources = vec![Resource("account:JohnSmith".to_string())];

        let (effect, policy) = evaluate(&policies, &action, &resources);

        assert_eq!(effect, Effect::Deny);
        assert_eq!(policy, Vec::<&Policy>::new());
    }

    #[test]
    fn it_denies_when_resource_does_not_match() {
        let policies = vec![Policy {
            actions: vec![Action("account:GetAccount".to_string())],
            effect: Effect::Allow,
            principals: vec![Principal("user:JohnSmith".to_string())],
            resources: vec![Resource("account:JohnSmith".to_string())],
        }];
        let action = Action("account:GetAccount".to_string());
        let resources = vec![Resource("account:JaneSmith".to_string())];

        let (effect, policy) = evaluate(&policies, &action, &resources);

        assert_eq!(effect, Effect::Deny);
        assert_eq!(policy, Vec::<&Policy>::new());
    }
}
