use crate::{Action, Effect, Policy, Resource};

pub fn is_authorized<'a>(
    policies: &'a [Policy],
    action: &Action,
    resources: &[Resource],
) -> (Effect, Vec<&'a Policy>) {
    let denied = policies
        .iter()
        .filter(|policy| {
            policy.effect == Effect::Deny
                && policy.actions.contains(action)
                && policy
                    .resources
                    .iter()
                    .any(|resource| resources.contains(resource))
        })
        .collect::<Vec<_>>();

    if !denied.is_empty() {
        return (Effect::Deny, denied);
    }

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
