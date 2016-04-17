#[macro_use]
extern crate mioco;
extern crate env_logger;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;

use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::str::FromStr;

use mioco::tcp::TcpListener;
use mioco::tcp::TcpStream;

type Registry = Arc<RwLock<HashMap<String, String>>>;

const DEFAULT_LISTEN_ADDR: &'static str = "127.0.0.1:5555";

fn listend_addr() -> SocketAddr {
    FromStr::from_str(DEFAULT_LISTEN_ADDR).unwrap()
}

fn main() {
    env_logger::init().unwrap();

    mioco::start(|| {
        let addr = listend_addr();
        let listener = TcpListener::bind(&addr).unwrap();
        println!("Starting tcp echo server on {:?}",
                 listener.local_addr().unwrap());

        let registry = Arc::new(RwLock::new(HashMap::<String, String>::new()));

        loop {
            let stream = listener.accept().unwrap();
            let registry = registry.clone();

            mioco::spawn(move || {
                ConnectionHandler::new(stream, registry).run();
            });
        }
    })
        .unwrap();
}

struct ConnectionHandler {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,

    status: ConnectionStatus,
    user: String,

    user_registry: Registry,
}

#[derive(PartialEq)]
enum ConnectionStatus {
    Connected,
    Login,
}

impl ConnectionHandler {
    fn new(stream: TcpStream, registry: Registry) -> ConnectionHandler {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream.try_clone().unwrap());

        ConnectionHandler {
            stream: stream,
            reader: reader,
            writer: writer,
            status: ConnectionStatus::Connected,
            user: "".to_string(),
            user_registry: registry,
        }
    }

    fn write(&mut self, msg: &[u8]) {
        self.writer.write(msg).unwrap();
        self.writer.flush().unwrap();
    }

    fn run(mut self) {
        if self.status == ConnectionStatus::Connected {
            self.login();
        }
        if self.status == ConnectionStatus::Login {
            self.command();
        }

        println!("it's over");
    }

    fn login(&mut self) {
        let mut timer = mioco::timer::Timer::new();
        timer.set_timeout(5000);

        select!(
            r:self.stream => {
                let mut line = String::new();
                self.reader
                    .read_line(&mut line)
                    // the format is "login $username $password"
                    .and_then( |_| {
                        if line.starts_with("login ") {
                            if let Some(user) = line.split_whitespace().nth(1) {
                                self.user = user.to_string();
                                self.user_registry
                                    .write()
                                    .unwrap()
                                    .insert(user.to_string(), "login1".to_string());
                                self.status = ConnectionStatus::Login;
                                self.write(b"login successfully\n");
                                println!("user \"{}\" login", user);
                                return Ok(());
                            }
                        }
                        Err(Error::new(ErrorKind::PermissionDenied, 
                            "user or password is invaild!"))
                    })
                    .or_else(|e| {
                        let msg = format!("login fail because of {:?}: {}\n", 
                            e.kind(), e);
                        print!("{}", msg);
                        self.write(msg.as_bytes());
                        Err(e)
                    })
                    .unwrap_or(());
            },
            r:timer => {
                println!("login timeout");
                self.write(b"login timeout\n");
            },
        )
    }

    fn command(mut self) {
        loop {
            println!("----sfasfasasf");
            let mut line = String::new();
            let result = self.reader.read_line(&mut line);

            match result {
                Ok(_) => {
                    match line.trim() {
                        "heartbreak" => self.write(b"got\n"),
                        "whoami" => {
                            let str = self.user.clone() + "\n";
                            self.write(str.as_bytes());
                        }
                        "quit" => {
                            self.write(b"bye\n");
                            break;
                        }
                        line => {
                            println!("{}", line);
                        }
                    }
                }
                Err(e) => println!("{:?}", e.kind()),
            }
        }
    }
}
