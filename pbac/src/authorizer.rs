use crate::action::ScopedAction;
use crate::{Effect, Element, Policy, ScopedResource};

pub fn is_authorized<'a>(
    policies: &'a [Policy],
    scoped_action: &ScopedAction,
    scoped_resources: &[ScopedResource],
) -> (Effect, Vec<&'a Policy>) {
    let policy_matches = policies
        .iter()
        .filter(|policy| {
            let action_match = policy
                .actions
                .iter()
                .any(|document| document.is_match(scoped_action));

            let resource_match = policy.resources.iter().any(|document| {
                scoped_resources
                    .iter()
                    .any(|scoped_resource| document.is_match(scoped_resource))
            });

            action_match && resource_match
        })
        .collect::<Vec<_>>();

    let denied_policy_matches = policy_matches
        .iter()
        .filter(|&policy| policy.effect == Effect::Deny)
        .copied()
        .collect::<Vec<_>>();

    if !denied_policy_matches.is_empty() {
        return (Effect::Deny, denied_policy_matches);
    }

    let allowed_policy_matches = policy_matches
        .iter()
        .filter(|&policy| policy.effect == Effect::Allow)
        .copied()
        .collect::<Vec<_>>();

    if !allowed_policy_matches.is_empty() {
        return (Effect::Allow, allowed_policy_matches);
    }

    (Effect::Deny, vec![])
}
