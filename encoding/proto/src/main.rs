mod proto;

use protobuf::Message;
use crate::proto::data::FooBar;

fn main() {
	let mut bar = FooBar::new();
	bar.set_name("test".to_string());
	bar.set_id(1);
	bar.set_email("test@abc.com".to_string());

	println!("old value = {:?}", bar);
	let bytes = bar.write_to_bytes().unwrap();

    let bar2: FooBar = Message::parse_from_bytes(bytes.as_slice()).unwrap();
    println!("new value = {:?}", bar2);

    assert_eq!(bar, bar2);
}
