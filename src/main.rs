use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("connection closed");
                break;
            }
            Ok(n) => {
                println!("received {} bytes", n);

                stream.write_all(b"+PONG\r\n").unwrap();
            }
            Err(e) => {
                println!("error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
     for stream in listener.incoming() {
        match stream {
             Ok(stream) => {
                 handle_connection(stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
