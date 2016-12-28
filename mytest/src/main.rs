use std::thread;
use std::sync::mpsc::channel;

fn main() {
    

    let (main_tx, main_rx) = channel();
    let mut thread_txs = vec![];
    
    for _ in 0..10 {
        let (thread_tx, thread_rx) = channel();
        thread_txs.push(thread_tx);

        let main_tx = main_tx.clone();

        thread::spawn(move|| {
            main_tx.send(thread_rx.recv().unwrap()).unwrap();
        });
    }

    for (i, tx) in thread_txs.iter().enumerate() {
        tx.send(i);
    }
    
    drop(main_tx);

    for i in main_rx.iter() {
        println!("{:?}", i);    
    }
    
}
