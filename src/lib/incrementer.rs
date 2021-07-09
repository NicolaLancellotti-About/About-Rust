// For see documentations:
// cargo doc --open

/// Function used by integration tests
///
/// # Examples
/// ```
/// let result = lib::increment(10);
/// assert_eq!(result, 11);
/// ```
/// # Errors
/// # Panics
/// # Safety
pub fn increment(value: i32) -> i32 {
    value + 1
}
