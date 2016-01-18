use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    run_server(handle_client);
}

fn run_server<F>(handle_client: F)
    // handle_client needs to be shared between threads. 
    // F's trait constraint is required.
    where F: 'static + Fn(TcpStream) + Send + Sync {

    let handle_client_arc = Arc::new(handle_client);

    match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            // accept connections and process them, spawning a new thread for each one
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let handle_client_clone = handle_client_arc.clone();
                        thread::spawn( move || {
                            // connection succeeded
                            handle_client_clone(stream);
                        });
                    }
                    Err(e) => { println!("{}", e); }
                }
            }

            // close the socket server
            drop(listener);

            println!("Exit!");
        } 
        Err(e) => {
            println!("{}", e);
        }
    }   
}

fn handle_client(stream: TcpStream) {
    // A TcpStream is unable to create BufReader and BufWriter at the same time.
    // So a cloned TcpStream is needed
    match stream.try_clone() {
        Ok(stream_clone) => {
            let mut writer = BufWriter::new(stream_clone);
            let reader = BufReader::new(stream);
            // a simple protocal for string message with line breaks
            for line in reader.lines() {
                // error handling in chained Results
                // see std::result::Result and std::io::Result
                if let Err(e) = line
                    // line is Result<String>. The String value is used to call closure
                    .and_then(|in_str| writer
                        .write_all(format!("receive: {}\n", in_str.trim()).as_bytes())
                    )
                    .and_then(|()| writer.flush()) {
                        println!("{}", e);
                }
            }
            println!("one connection is closed");
        } 
        Err(e) => {
            println!("Opening connection is failed. {}", e);    
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }
}