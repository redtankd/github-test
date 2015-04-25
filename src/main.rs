use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    fn handle_client(stream: TcpStream) {
        let writer_stream = stream.try_clone().unwrap();
        let reader = BufReader::new(stream);
        let mut writer = BufWriter::new(writer_stream);

        for line in reader.lines() {
            let in_str = line.unwrap();
            println!("{}", in_str);
            writer.write_fmt(format_args!("receive: {}\n", in_str.trim()));
        }

        // let mut in_str = String::new();
        // let read_size = buffer.read_line(&mut in_str).unwrap();
        // println!("{}", in_str);

        // buffer.write_fmt(format_args!("receive: {}\nsize: {}\n", in_str.trim(), read_size));
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn( move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { println!("{}", e); }
        }
    }

    // close the socket server
    drop(listener);

    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }
}