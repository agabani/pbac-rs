use pbac::{is_authorized, Action, Effect, Policy};

#[test]
fn it_implicitly_denies_when_no_policies_are_provided() {
    let polices = vec![];
    let action = Action("noop".to_string());
    let resources = vec![];

    let (effect, policy) = is_authorized(&polices, &action, &resources);

    assert_eq!(effect, Effect::Deny);
    assert_eq!(policy, Vec::<&Policy>::new());
}
