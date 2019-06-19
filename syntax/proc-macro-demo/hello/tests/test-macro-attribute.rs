use hello::HelloMacro;

#[hello_macro::hello]
fn wrapped_function() {}

#[hello_macro::struct_extension]
struct DummyStruct {
    my_field: i32,
}

#[hello_macro::impl_trait]
impl HelloMacro for DummyStruct {}

struct DummyStruct2;

impl HelloMacro for DummyStruct2 {
    hello_macro::func_macro!("hello");
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

    assert_eq!("hello".to_owned(), DummyStruct::hello_macro());

    assert_eq!("hello".to_owned(), DummyStruct2::hello_macro());
}
