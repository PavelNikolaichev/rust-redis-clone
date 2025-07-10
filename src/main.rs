use dotenv::dotenv;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use log::{info, error};


async fn handle_connection(mut stream: TcpStream) {
    info!("accepted new connection: {:?}", stream.peer_addr());

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                info!("connection closed by client: {:?}", stream.peer_addr());
                return;
            }
            Ok(n) => {
                info!("received {} bytes from {:?}", n, stream.peer_addr());

                if let Err(e) = stream.write_all(b"+PONG\r\n").await {
                    error!("error writing to stream: {}", e);
                    return;
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
