use dotenv::dotenv;
use log::{error, info};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub mod resp;
use resp::command_dispatcher::CommandDispatcher;
use resp::protocol::{deserialize, serialize, RespType};
use crate::resp::state::default_server_state::DefaultServerState;

async fn handle_connection(
    mut stream: TcpStream,
    dispatcher: Arc<CommandDispatcher>,
    state: Arc<Mutex<DefaultServerState>>,
) {
    let mut buffer = vec![0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                // Connection closed
                info!("Connection closed by client");
                return;
            }
            Ok(n) => {
                let input = &buffer[..n];
                match deserialize(input) {
                    Ok((resp_type, _)) => {
                        let command_name = match resp_type {
                            RespType::Array(ref arr) if !arr.is_empty() => {
                                if let RespType::BulkString(Some(ref cmd)) = arr[0] {
                                    cmd.to_uppercase()
                                } else {
                                    error!("Invalid command format");
                                    continue;
                                }
                            }
                            _ => {
                                error!("Invalid command format");
                                continue;
                            }
                        };

                        let args = match resp_type {
                            RespType::Array(arr) => {
                                arr[1..].iter().cloned().collect::<Vec<RespType>>()
                            }
                            _ => Vec::new(),
                        };
                        let mut guard = state.lock().await;
                        match dispatcher.dispatch(&command_name, args, &mut *guard) {
                            Ok(response) => {
                                let response_bytes = serialize(&response);
                                if let Err(e) = stream.write_all(&response_bytes).await {
                                    error!("Failed to write response: {}", e);
                                    return;
                                }
                            }
                            Err(e) => {
                                error!("Command execution failed: {}", e);
                                let error_response = RespType::Error(e);
                                let response_bytes = serialize(&error_response);
                                if let Err(e) = stream.write_all(&response_bytes).await {
                                    error!("Failed to write error response: {}", e);
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse request: {}", e);
                        return;
                    }
                }
            }
            Err(e) => {
                error!("Failed to read from stream: {}", e);
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

    let dispatcher = Arc::new(CommandDispatcher::new());
    let state = Arc::new(Mutex::new(DefaultServerState::default()));

    // TODO: use dotenv to load configuration
    info!("Starting Redis server on port 6379...");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let d = Arc::clone(&dispatcher);
                let s = Arc::clone(&state);
                tokio::spawn(async move {
                    handle_connection(stream, d, s).await;
                });
            }
            Err(e) => {
                error!("error accepting connection: {}", e);
            }
        }
    }
}
