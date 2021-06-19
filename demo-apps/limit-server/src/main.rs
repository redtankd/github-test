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

use std::thread;
use std::time::Duration;

use actix_rt::System;
use actix_web::{web, App, HttpResponse, HttpServer};

fn main() -> std::io::Result<()> {
    let runner = System::new();
    let sys = System::current();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        println!("System is required to stop!");
        sys.stop();
    });

    runner.block_on(async move {
        HttpServer::new(|| {
            App::new().service(web::resource("/").to(|| HttpResponse::Ok().body("data")))
        })
        .shutdown_timeout(5)
        .workers(1)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
    });

    runner.run()
}

#[cfg(all(feature = "nightly", test))]
mod bench {
    use super::*;
    use actix_rt::System;
    use actix_web::{web, App, HttpResponse, HttpServer};
    use http::StatusCode;
    use std::io;
    use std::sync::{mpsc, Once};
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
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let runner = System::new();

            // I think block_on() likes spawn() more.
            runner.block_on(async move {
                HttpServer::new(|| {
                    App::new().service(web::resource("/").to(|| HttpResponse::Ok().body("ok")))
                })
                .shutdown_timeout(5)
                .workers(1)
                .bind("127.0.0.1:8080")
                .unwrap()
                .run();
            });

            let _ = tx.send(actix_rt::System::current());

            let _ = runner.run();
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

        let sys = rx.recv().unwrap();
        sys.stop();

        Ok(())
    }
}
