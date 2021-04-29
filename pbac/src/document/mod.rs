mod action;
mod resource;
mod wildcard;

use crate::ElementParseError;
pub use action::ActionDocument;
pub use resource::ResourceDocument;

pub trait Element<T>
where
    Self: Sized,
{
    fn is_match(&self, value: &T) -> bool;
    fn parse(value: &str) -> Result<Self, ElementParseError>;
}
