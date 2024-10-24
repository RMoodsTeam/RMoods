use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use log::{info, warn};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::WebSocketStream;

#[derive(Debug)]
pub enum WebSocketMessage {
    NewRequestLimit(u16),
    ReportDone(()),
    Ping(String),
}

async fn authenticate(mut ws: SplitStream<WebSocketStream<TcpStream>>) -> anyhow::Result<String> {
    let msg = ws.next().await.unwrap()?.to_string();
    warn!("Authenticating: {}", msg);
    Ok(msg)
}

async fn accept_connection(
    (stream, socket_addr): (TcpStream, std::net::SocketAddr),
    peers: &mut HashMap<
        String,
        SplitSink<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::protocol::Message>,
    >,
) {
    info!("New connection from: {}", socket_addr.to_string());
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");
    let (write, read) = ws_stream.split();

    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            info!("Ping timeout");
        },
        res = authenticate(read) => {
            match res {
                Ok(email) => {
                    info!("Authenticated email {email}");
                    peers.insert(email, write);
                    info!("Peers: {}", peers.len());
                },
                Err(e) => {
                    warn!("Failed to authenticate: {}", e);
                }
            }
        }
    }
}

async fn handle_system_message(msg: WebSocketMessage) {
    match msg {
        WebSocketMessage::NewRequestLimit(limit) => {
            info!("New request limit: {}", limit);
        }
        WebSocketMessage::ReportDone(()) => {
            info!("Report done");
        }
        WebSocketMessage::Ping(msg) => {
            info!("Ping: {}", msg);
        }
    }
}

pub async fn start_service(port: u16, mut rx: Receiver<WebSocketMessage>) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    let mut peers: HashMap<
        String,
        SplitSink<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::protocol::Message>,
    > = HashMap::new();

    info!("WebSocket service listening on: {}", addr);

    // Some frontend client connects to this WebSocket service here
    while let Ok((stream, socket_addr)) = listener.accept().await {
        tokio::select! {
            _ = accept_connection((stream, socket_addr), &mut peers) => {
                info!("Connection accepted");
            },
            msg = rx.recv() => {
                info!("Received message: {:?}", msg);
                if let Some(msg) = msg {
                    handle_system_message(msg).await;
                }
            }
        }
    }

    Ok(())
}
