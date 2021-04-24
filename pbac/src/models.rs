use crate::document::ActionDocument;

/// The Effect describes the specific effect.
#[derive(Debug, PartialEq)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, PartialEq)]
pub struct Policy {
    /// The Action element describes the specific action or actions that will be allowed or denied.
    pub actions: Vec<ActionDocument>,

    /// The Effect element is required and specifies whether the statement results in an allow or an explicit deny.
    pub effect: Effect,

    /// Use the Principal element in a policy to specify the principal that is allowed or denied access to a resource.
    pub principals: Vec<Principal>,

    /// The Resource element specifies the object or objects that the statement covers.
    pub resources: Vec<Resource>,
}

/// The Principal describes the specific principal.
#[derive(Debug, PartialEq)]
pub struct Principal(pub String);

/// The Resource describes the specific resource.
#[derive(Debug, PartialEq)]
pub struct Resource(pub String);
