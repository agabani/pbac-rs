use pbac::{
    is_authorized, ActionDocument, Effect, Element, Policy, ResourceDocument, ScopedAction,
    ScopedResource,
};

#[test]
fn implicit_deny_when_no_policies() {
    let policies = vec![];

    let action = ScopedAction::parse("scope:verb:resource");
    let resources = vec![ScopedResource::parse("scope:resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, Vec::<&Policy>::new());
}

#[test]
fn explicit_allow_if_action_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("scope:verb:resource")],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("*")],
    }];

    let action = ScopedAction::parse("scope:verb:resource");
    let resources = &vec![ScopedResource::parse("scope:resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Allow);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn explicit_deny_if_action_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("scope:verb:resource")],
        effect: Effect::Deny,
        principals: vec![],
        resources: vec![ResourceDocument::parse("*")],
    }];

    let action = ScopedAction::parse("scope:verb:resource");
    let resources = &vec![ScopedResource::parse("scope:resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn implicit_deny_if_action_not_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("scope:verb:resource")],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("*")],
    }];

    let action = ScopedAction::parse("scope:verb:other-resource");
    let resources = vec![ScopedResource::parse("scope:resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, Vec::<&Policy>::new());
}

#[test]
fn explicit_allow_if_resource_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("*")],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("scope:resource")],
    }];

    let action = ScopedAction::parse("scope:verb:resource");
    let resources = vec![ScopedResource::parse("scope:resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Allow);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn explicit_deny_if_resource_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("*")],
        effect: Effect::Deny,
        principals: vec![],
        resources: vec![ResourceDocument::parse("scope:resource")],
    }];

    let action = ScopedAction::parse("scope:verb:resource");
    let resources = vec![ScopedResource::parse("scope:resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, &[policies[0]]);
}

#[test]
fn implicit_deny_if_resource_not_match() {
    let policies = vec![Policy {
        actions: vec![ActionDocument::parse("*")],
        effect: Effect::Allow,
        principals: vec![],
        resources: vec![ResourceDocument::parse("scope:resource")],
    }];

    let action = ScopedAction::parse("scope:verb:resource");
    let resources = vec![ScopedResource::parse("scope:other-resource")];

    let (effect, policies) = is_authorized(&policies, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policies, Vec::<&Policy>::new());
}
