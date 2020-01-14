// use tokio::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, Weak};
use std::thread;

use slab::Slab;

#[derive(Debug)]
struct Order {
    id: u32,
    price: u64,
}

struct OrderSlot {
    order: Order,
    ty: u8,
    next: Option<Weak<Mutex<OrderSlot>>>,
}

pub struct OrderBook {
    slots: Slab<Arc<Mutex<OrderSlot>>>,
}

impl OrderBook {
    pub fn with_capacity(capacity: usize) -> OrderBook {
        OrderBook {
            slots: Slab::with_capacity(capacity),
        }
    }

    // fn get(&self, key: usize) -> Arc<Mutex<Order>> {
    //     self.slots.get(key).unwrap().clone()
    // }
}

#[test]
fn test_weak() {
    let mut order_book = OrderBook::with_capacity(1000);

    let order1 = Order { id: 1, price: 100 };
    let order_slot1 = Arc::new(Mutex::new(OrderSlot {
        order: order1,
        ty: 1,
        next: None,
    }));

    let order2 = Order { id: 2, price: 100 };
    let order_slot2 = Arc::new(Mutex::new(OrderSlot {
        order: order2,
        ty: 1,
        next: Some(Arc::downgrade(&order_slot1)),
    }));

    let order_key1 = order_book.slots.insert(order_slot1);
    let order_key2 = order_book.slots.insert(order_slot2);

    assert_eq!(1, order_slot1.ty);
}

fn main() {
    let mut slots = Slab::<Arc<Mutex<Order>>>::with_capacity(10_1000);
    let order1 = Arc::new(Mutex::new(Order { id: 1, price: 100 }));
    let key = slots.insert(order1);

    let (tx, rx) = channel::<Arc<Mutex<Order>>>();

    let handle = thread::spawn(move || {
        let order1_l = rx.recv().unwrap();
        let mut order1_l = order1_l.lock().unwrap();
        println!("{}", order1_l.id);
        order1_l.id = 2;
    });

    let _ = tx.send(slots.get(key).unwrap().clone());

    let _ = handle.join();

    let order1 = slots.get(key).unwrap();

    println!("{}", order1.lock().unwrap().id);
}
