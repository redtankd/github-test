#[hello_macro::hello]
fn wrapped_function() {}

#[test]
fn works() {
   assert_eq!(42, wrapped_function());
}