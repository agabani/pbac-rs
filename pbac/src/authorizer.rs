use crate::{Effect, Policy};

pub fn is_authorized<'a>() -> (Effect, Vec<&'a Policy>) {
    (Effect::Deny, vec![])
}
