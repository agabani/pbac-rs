use crate::action::ScopedAction;
use crate::{Effect, Policy};

pub fn is_authorized<'a>(
    policies: &'a [Policy],
    scoped_action: &ScopedAction,
) -> (Effect, Vec<&'a Policy>) {
    let any_allowed = policies
        .iter()
        .filter(|policy| {
            policy
                .actions
                .iter()
                .any(|document| document.is_match(scoped_action))
        })
        .collect::<Vec<_>>();

    if !any_allowed.is_empty() {
        return (Effect::Allow, any_allowed);
    }

    (Effect::Deny, vec![])
}
