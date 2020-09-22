/// A data structure the exposes the number of fields it has.
///
/// This trait can be derived:
///
/// ```
/// use field_count::FieldCount;
///
/// #[derive(FieldCount)]
/// struct MyStruct
/// {
///    first_field: i32,
///    second_field: String,
///    third_field: u16,
/// }
///
/// println!("{}", MyStruct::field_count()); // 3
/// ```
pub trait FieldCount {
    /// Get the number of fields on a struct.
    fn field_count() -> usize;
}

// Export derive macro from derive crate.
pub use field_count_derive::*;

#[cfg(test)]
mod tests {
    use super::FieldCount;

    #[derive(FieldCount)]
    struct MyStruct {
        _first_field: i32,
        _second_field: String,
        _third_field: u16,
    }

    #[test]
    fn test_derive_field_count() {
        assert_eq!(MyStruct::field_count(), 3);
    }
}
