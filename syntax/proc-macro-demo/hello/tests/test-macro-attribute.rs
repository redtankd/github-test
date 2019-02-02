#[hello_macro::hello]
fn wrapped_function() {}

#[hello_macro::struct_extension]
struct DummyStruct {
    my_field: i32,
}

#[test]
fn works() {
    assert_eq!(42, wrapped_function());

    let dummy_struct = DummyStruct {
        my_field: 32,
        append: String::from("someone@example.com"),
    };
    assert_eq!(32, dummy_struct.my_field);
    assert_eq!(String::from("someone@example.com"), dummy_struct.append);
}
