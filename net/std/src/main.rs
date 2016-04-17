use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Duration;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    let registry = Arc::new(RwLock::new(HashMap::<String, String>::new()));

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
	        	let registry = registry.clone();
	            thread::spawn(move|| {
	                // connection succeeded
					Handler::new(stream, registry).handle()
	            });
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}

	// close the socket server
	drop(listener);
}

struct Handler {
	stream: TcpStream,
	reader: BufReader<TcpStream>,
	writer: BufWriter<TcpStream>,
	connection_status: ConnectionStatus,
	user: String,
	user_registry: Arc<RwLock<HashMap<String, String>>>
}

#[derive(PartialEq)]
enum ConnectionStatus {
	Connected,
	Login,
}

impl Handler {
	fn new(stream: TcpStream, registry: Arc<RwLock<HashMap<String, String>>>) -> Handler {
		let reader = BufReader::new(stream.try_clone().unwrap());
		let writer = BufWriter::new(stream.try_clone().unwrap());
		// stream.set_nonblocking(true);

		Handler {
			stream: stream,
			reader: reader,
			writer: writer,
			connection_status: ConnectionStatus::Connected,
			user: "".to_string(),
			user_registry: registry
		}
	}

	fn write(&mut self, msg: &[u8]) {
		self.writer.write(msg);
		self.writer.flush();
	}

	fn handle(mut self) {
		if self.connection_status == ConnectionStatus::Connected { self.login(); }
		if self.connection_status == ConnectionStatus::Login { self.command(); }

		println!("it's over");
	}

	fn login(&mut self) {
		// need to login in 10s
		self.stream.set_read_timeout(Some(Duration::new(10, 0)));

		// login 5 times at maximum
		for _ in 0..5 {
			self.write(b"login\n");

			let mut line = String::new();
			let result = self.reader.read_line(&mut line);

			match result {
			    Ok(_) => {
			    	if line.starts_with("login") {
			    		if let Some(user) = line.split_whitespace().nth(1) {
			    			self.user = user.to_string();
			    			self
				    			.user_registry
				    			.write()
				    			.unwrap()
								.insert(user.to_string(), "login1".to_string());
							self.connection_status = ConnectionStatus::Login;
							self.write(b"login successfully\n");
							println!("user \"{}\" login", user);
							break; 		
			    		}
			    	}
			    },
			    Err(e) => {
			    	println!("{:?}", e.kind());
			    },
			}
		}
		
		if self.connection_status == ConnectionStatus::Connected {
			self.write(b"login failed\n"); 
		}
	}

	fn command(mut self) {
		loop {
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
				    	"quit" => { self.write(b"bye\n"); break; }
					    line => {
					    	println!("{}", line);
					    }
					}
				}
			    Err(e) => println!("{:?}", e.kind())
			}
		}
	}
}