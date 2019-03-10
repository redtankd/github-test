#![feature(await_macro, async_await, futures_api)]

use futures::executor::{self, ThreadPool};
use futures::prelude::*;
use futures::task::SpawnExt;

use romio::{TcpListener, TcpStream};

use std::net::SocketAddr;

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 1024];

    loop {
        match await!(stream.read(&mut buf))? {
            0 => break, // Socket closed
            n => {
                // Send the data back
                await!(stream.write_all(&buf[0..n]))?;
            }
        }
    }

    Ok(())
}

fn main() {
    use std::env;

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8000".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Bind the TCP listener
    let mut listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    let mut incoming = listener.incoming();

    // the thread pool to handle client connection
    let mut threadpool = ThreadPool::new().unwrap();

    executor::block_on(
        async {
            while let Some(stream) = await!(incoming.next()) {
                let stream = stream.unwrap();
                threadpool
                    .spawn(handle_client(stream).map(|x| x.unwrap()))
                    .unwrap();
            }
        },
    );
}
