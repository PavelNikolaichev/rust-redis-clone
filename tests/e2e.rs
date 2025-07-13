use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::sync::Once;
use std::thread::sleep;
use std::time::Duration;

fn start_server() -> Child {
    Command::new("cargo")
        .args(&["run"])
        .spawn()
        .expect("failed to start server")
}

/// Waits until the server is ready to accept connections, or times out after 10 seconds.
fn wait_for_server_ready(addr: &str) {
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        if TcpStream::connect(addr).is_ok() {
            return;
        }
        sleep(Duration::from_millis(100));
    }
    panic!("Server did not start in time");
}

static START: Once = Once::new();

fn start_server_once() {
    START.call_once(|| {
        let mut child = start_server();
        wait_for_server_ready("127.0.0.1:6379");
        // Leak the child so it lives for the duration of the tests
        std::mem::forget(child);
    });
}

fn send_and_receive(stream: &mut TcpStream, req: &[u8]) -> Vec<u8> {
    stream.write_all(req).unwrap();
    let mut buf = [0; 128];
    let n = stream.read(&mut buf).unwrap();
    buf[..n].to_vec()
}

#[test]
fn test_ping_e2e() {
    start_server_once();
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let response = send_and_receive(&mut stream, b"*1\r\n$4\r\nPING\r\n");
    assert_eq!(response, b"+PONG\r\n");
}

#[test]
fn test_echo_e2e() {
    start_server_once();
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let response = send_and_receive(&mut stream, b"*2\r\n$4\r\nECHO\r\n$13\r\nHello, World!\r\n");
    assert_eq!(response, b"$13\r\nHello, World!\r\n");
}

#[test]
fn simultaneous_connections() {
    start_server_once();
    let mut stream1 = TcpStream::connect("127.0.0.1:6379").unwrap();
    let mut stream2 = TcpStream::connect("127.0.0.1:6379").unwrap();
    let response1 = send_and_receive(&mut stream1, b"*1\r\n$4\r\nPING\r\n");
    let response2 = send_and_receive(&mut stream2, b"*1\r\n$4\r\nPING\r\n");
    assert_eq!(response1, b"+PONG\r\n");
    assert_eq!(response2, b"+PONG\r\n");
    assert!(stream1.shutdown(std::net::Shutdown::Both).is_ok());
    assert!(stream2.shutdown(std::net::Shutdown::Both).is_ok());
}

#[test]
fn test_multiple_commands() {
    start_server_once();
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let response = send_and_receive(&mut stream, b"*1\r\n$4\r\nPING\r\n");
    assert_eq!(response, b"+PONG\r\n");
    let response = send_and_receive(&mut stream, b"*2\r\n$4\r\nECHO\r\n$13\r\nHello, World!\r\n");
    assert_eq!(response, b"$13\r\nHello, World!\r\n");
    let response = send_and_receive(&mut stream, b"*1\r\n$4\r\nPING\r\n");
    assert_eq!(response, b"+PONG\r\n");
    assert!(stream.shutdown(std::net::Shutdown::Both).is_ok());
}
