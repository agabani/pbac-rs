mod action;
mod authorizer;
mod document;
mod models;
mod resource;

pub use crate::action::ScopedAction;
pub use crate::authorizer::is_authorized;
pub use crate::document::{ActionDocument, Element, ResourceDocument};
pub use crate::models::{Effect, Policy, Principal};
pub use crate::resource::ScopedResource;

#[derive(Debug, PartialEq)]
pub struct ElementParseError {
    pub token: String,
}
