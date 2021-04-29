use pbac::{
    is_authorized, ActionDocument, Effect, Element, Policy, ResourceDocument, ScopedAction,
    ScopedResource,
};

#[test]
fn implicit_deny_when_no_policies() {
    let policies = vec![];

    let action = ScopedAction::parse("scope:verb:resource").unwrap();
    let resources = vec![ScopedResource::parse("scope:resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, Vec::<&Policy>::new());
}

#[test]
fn explicit_allow_if_action_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("scope:verb:resource").unwrap()],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("*").unwrap()],
    }];

    let action = ScopedAction::parse("scope:verb:resource").unwrap();
    let resources = &vec![ScopedResource::parse("scope:resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Allow);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn explicit_deny_if_action_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("scope:verb:resource").unwrap()],
        effect: Effect::Deny,
        principals: vec![],
        resources: vec![ResourceDocument::parse("*").unwrap()],
    }];

    let action = ScopedAction::parse("scope:verb:resource").unwrap();
    let resources = &vec![ScopedResource::parse("scope:resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn implicit_deny_if_action_not_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("scope:verb:resource").unwrap()],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("*").unwrap()],
    }];

    let action = ScopedAction::parse("scope:verb:other-resource").unwrap();
    let resources = vec![ScopedResource::parse("scope:resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, Vec::<&Policy>::new());
}

#[test]
fn explicit_allow_if_resource_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("*").unwrap()],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("scope:resource").unwrap()],
    }];

    let action = ScopedAction::parse("scope:verb:resource").unwrap();
    let resources = vec![ScopedResource::parse("scope:resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Allow);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn explicit_deny_if_resource_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("*").unwrap()],
        effect: Effect::Deny,
        principals: vec![],
        resources: vec![ResourceDocument::parse("scope:resource").unwrap()],
    }];

    let action = ScopedAction::parse("scope:verb:resource").unwrap();
    let resources = vec![ScopedResource::parse("scope:resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn implicit_deny_if_resource_not_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("*").unwrap()],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("scope:resource").unwrap()],
    }];

    let action = ScopedAction::parse("scope:verb:resource").unwrap();
    let resources = vec![ScopedResource::parse("scope:other-resource").unwrap()];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, Vec::<&Policy>::new());
}
