#![cfg_attr(all(feature = "nightly", test), feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

// use std::sync::mpsc::channel;
// use std::sync::{Arc, Mutex, Weak};
// use std::thread;

// use slab::Slab;

// #[derive(Debug)]
// struct Order {
//     id: u32,
//     price: u64,
// }

// struct OrderSlot {
//     order: Order,
//     ty: u8,
//     next: Option<Weak<Mutex<OrderSlot>>>,
// }

// pub struct OrderBook {
//     slots: Slab<Arc<Mutex<OrderSlot>>>,
// }

// impl OrderBook {
//     pub fn with_capacity(capacity: usize) -> OrderBook {
//         OrderBook {
//             slots: Slab::with_capacity(capacity),
//         }
//     }

//     // fn get(&self, key: usize) -> Arc<Mutex<Order>> {
//     //     self.slots.get(key).unwrap().clone()
//     // }
// }

// #[test]
// fn test_weak() {
//     let mut order_book = OrderBook::with_capacity(1000);

//     let order1 = Order { id: 1, price: 100 };
//     let order_slot1 = Arc::new(Mutex::new(OrderSlot {
//         order: order1,
//         ty: 1,
//         next: None,
//     }));

//     let order2 = Order { id: 2, price: 100 };
//     let order_slot2 = Arc::new(Mutex::new(OrderSlot {
//         order: order2,
//         ty: 1,
//         next: Some(Arc::downgrade(&order_slot1)),
//     }));

//     let order_key1 = order_book.slots.insert(order_slot1);
//     let order_key2 = order_book.slots.insert(order_slot2);

//     // assert_eq!(1, order_slot1.lock().unwrap().ty);
// }

fn main() {
    //     let mut slots = Slab::<Arc<Mutex<Order>>>::with_capacity(10_1000);
    //     let order1 = Arc::new(Mutex::new(Order { id: 1, price: 100 }));
    //     let key = slots.insert(order1);

    //     let (tx, rx) = channel::<Arc<Mutex<Order>>>();

    //     let handle = thread::spawn(move || {
    //         let order1_l = rx.recv().unwrap();
    //         let mut order1_l = order1_l.lock().unwrap();
    //         println!("{}", order1_l.id);
    //         order1_l.id = 2;
    //     });

    //     let _ = tx.send(slots.get(key).unwrap().clone());

    //     let _ = handle.join();

    //     let order1 = slots.get(key).unwrap();

    //     println!("{}", order1.lock().unwrap().id);
}

#[cfg(all(feature = "nightly", test))]
mod bench {
    use super::*;
    use actix_rt::System;
    use actix_web::{web, App, HttpResponse, HttpServer};
    use http::StatusCode;
    use std::io;
    use std::sync::Once;
    use std::thread;
    use test::Bencher;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            let _ = env_logger::init();
        });
    }

    #[bench]
    fn bench_http_server(b: &mut Bencher) -> io::Result<()> {
        setup();

        thread::spawn(move || {
            let sys = System::new("http-server");
            let _server =
                HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
                    .bind("127.0.0.1:8080")
                    .unwrap()
                    .shutdown_timeout(1)
                    .run();
            let _ = sys.run();
        });

        thread::sleep(std::time::Duration::from_millis(100));

        b.iter(|| {
            assert!(reqwest::blocking::get("http://127.0.0.1:8080")
                .and_then(|response| {
                    assert_eq!(StatusCode::OK, response.status());
                    Ok(())
                })
                .is_ok());
        });

        Ok(())
    }
}
