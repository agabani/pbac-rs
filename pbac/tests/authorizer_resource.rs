use pbac::{is_authorized, Action, Effect, Policy, Principal, Resource};

#[test]
fn it_explicitly_allows_when_resource_does_match() {
    let policies = vec![Policy {
        actions: vec![Action("account:GetAccount".to_string())],
        effect: Effect::Allow,
        principals: vec![Principal("user:JohnSmith".to_string())],
        resources: vec![Resource("account:JohnSmith".to_string())],
    }];
    let action = Action("account:GetAccount".to_string());
    let resources = vec![Resource("account:JohnSmith".to_string())];

    let (effect, policy) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Allow);
    assert_eq!(policy, &[policy[0]]);
}

#[test]
fn it_explicitly_denies_when_resource_does_match() {
    let policies = vec![Policy {
        actions: vec![Action("account:GetAccount".to_string())],
        effect: Effect::Deny,
        principals: vec![Principal("user:JohnSmith".to_string())],
        resources: vec![Resource("account:JohnSmith".to_string())],
    }];
    let action = Action("account:GetAccount".to_string());
    let resources = vec![Resource("account:JohnSmith".to_string())];

    let (effect, policy) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policy, &[policy[0]]);
}

#[test]
fn it_implicitly_denies_when_resource_does_not_match() {
    let policies = vec![Policy {
        actions: vec![Action("account:GetAccount".to_string())],
        effect: Effect::Allow,
        principals: vec![Principal("user:JohnSmith".to_string())],
        resources: vec![Resource("account:JohnSmith".to_string())],
    }];
    let action = Action("account:GetAccount".to_string());
    let resources = vec![Resource("account:JaneSmith".to_string())];

    let (effect, policy) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policy, Vec::<&Policy>::new());
}
