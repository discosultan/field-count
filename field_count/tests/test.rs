use field_count::FieldCount;

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
