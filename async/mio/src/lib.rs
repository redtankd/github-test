
#[cfg(test)] 
use mio::{Events, Ready, Poll, PollOpt, Token};

#[cfg(test)] 
use mio::tcp::{TcpListener, TcpStream};

#[cfg(test)] 
use std::io::{Write, Read};

#[test]
fn test() {
    // Setup some tokens to allow us to identify which event is
    // for which socket.
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    let addr = "127.0.0.1:13265".parse().unwrap();

    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // Create an poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();

    // Setup the client socket
    let mut sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge())
        .unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let (mut conn, _) = server.accept().unwrap();
                    conn.write(String::from("Hello, world!").as_bytes()).unwrap();
                }
                CLIENT => {
                    // The server just shuts down the socket, let's just exit
                    // from our event loop.
                    let mut str = String::new();
                    sock.read_to_string(&mut str).unwrap();
                    assert_eq!("Hello, world!", str);

                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}
