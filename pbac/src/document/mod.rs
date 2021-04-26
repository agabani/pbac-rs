mod action;
mod resource;
mod wildcard;

pub use action::ActionDocument;
pub use resource::ResourceDocument;

pub trait Element<T> {
    fn is_match(&self, value: &T) -> bool;
    fn parse(value: &str) -> Self;
}
