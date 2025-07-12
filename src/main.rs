use dotenv::dotenv;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use log::{info, error};


async fn handle_command(command: &str, stream: &mut TcpStream) {
    let line = command.trim();
    info!("handling command: {}", line);

    let mut args = line.split_whitespace();

    match args.next().map(|cmd| cmd.to_uppercase()) {
        Some(ref cmd) if cmd == "PING" => {
            info!("PING command received");
            let resp = format!("${}\r\n{}\r\n", 4, "PONG");
            if let Err(e) = stream.write_all(resp.as_bytes()).await {
                error!("error writing to stream: {}", e);
            }
        }
        Some(ref cmd) if cmd == "ECHO" => {
            let message = args.next().unwrap_or("");
            info!("ECHO command received with message: {}", message);

            let resp = format!("${}\r\n{}\r\n", message.len(), message);
            if let Err(e) = stream.write_all(resp.as_bytes()).await {
                error!("error writing to stream: {}", e);
            }
        }
        _ => {
            error!("unknown command: {}", line);
            if let Err(e) = stream.write_all(b"-ERR unknown command\r\n").await {
                error!("error writing to stream: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    info!("accepted new connection: {:?}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                info!("connection closed by client: {:?}", stream.peer_addr().unwrap());
                return;
            }
            Ok(n) => {
                info!("received {} bytes from {:?}", n, stream.peer_addr().unwrap());

                match std::str::from_utf8(&buffer[0..n]) {
                    Ok(command) => {
                        info!("received command: {}", command.trim());

                        // Process the command
                        handle_command(command.trim(), &mut stream).await;
                    }
                    Err(e) => {
                        error!("invalid UTF-8 sequence: {}", e);
                        return;
                    }
                }
            }
            Err(e) => {
                error!("error reading from stream: {}", e);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(e) => {
                error!("error accepting connection: {}", e);
            }
        }
    }
}
