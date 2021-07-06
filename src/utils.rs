use std::any::type_name;

/// Check the type of a value
pub fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}