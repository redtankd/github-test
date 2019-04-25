#![feature(await_macro, async_await)]

use futures::prelude::*;

use romio::TcpStream;

use std::net::SocketAddr;

const MESSAGES: &[&str] = &["hello", "world", "one two three"];

async fn run_client(addr: SocketAddr) -> std::io::Result<()> {
    let mut stream = await!(TcpStream::connect(&addr))?;
    println!("Connected");

    // Buffer to read into
    let mut buf = [0; 128];

    for msg in MESSAGES {
        println!(" > write = {:?}", msg);

        // Write the message to the server
        await!(stream.write_all(msg.as_bytes()))?;

        // Read the message back from the server
        await!(stream.read(&mut buf))?;

        assert_eq!(&buf[..msg.len()], msg.as_bytes());
    }

    Ok(())
}

fn main() {
    use std::env;

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8000".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Connect to the echo server
    futures::executor::block_on(
        async {
            match await!(run_client(addr)) {
                Ok(_) => println!("done."),
                Err(e) => eprintln!("echo client failed; error = {:?}", e),
            }
        },
    );
}
