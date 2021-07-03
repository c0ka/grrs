use std::any::{type_name};
pub use single_linked::List;

mod single_linked;
mod persis_linked;
mod deque_linked;

/// Check the type of a value
pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}