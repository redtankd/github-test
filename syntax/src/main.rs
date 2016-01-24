use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

mod ownership;
mod deref_coercions;
mod mytraits;

fn main() {
    let mut hashmap = HashMap::<&str, Rc<Any>>::new();

    // hashmap is &mut borrowed in insert(), but the scope is ended in the statement
    hashmap.insert("resource", Rc::new("abc"));

    // So hashmap can be &mut borrowed twice
    new_instance(&mut hashmap);

    // so hashmap can be borrowed
    // but hashmap can't be &mut borrowed from here
    let tx1 = hashmap.get("inst1").unwrap().downcast_ref::<Sender<i32>>().unwrap();
    println!("Sender: {:?}", tx1.send(100));

    // waiting the child thread 
    std::thread::sleep_ms(100);
}

fn new_instance(hashmap: &mut HashMap<&str, Rc<Any>>) {
	let resource: &str;
	{
		// hashmap is &mut borrowed in outter scope and can't be read-only borrowed.
		// So a new scope to avoid confliting scopes 
		let re1 = hashmap.get("resource").unwrap().downcast_ref::<&str>().unwrap();
		resource = re1;
	}

	let (tx, rx) = channel::<i32>();

	thread::spawn(move|| {
		println!("Receiver: waiting");
    	let j1 = rx.recv().unwrap();
    	println!("{} and {:?}", resource, j1);
	});

	hashmap.insert("inst1", Rc::new(tx.clone()));
}