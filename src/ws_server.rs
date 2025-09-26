use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info, warn};

type ClientId = usize;

pub struct WsServer {
    clients: Arc<DashMap<ClientId, mpsc::UnboundedSender<String>>>,
    next_client_id: Arc<std::sync::atomic::AtomicUsize>,
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(DashMap::new()),
            next_client_id: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    pub async fn run(&self, listener: TcpListener) {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New client connected from: {}", addr);
                    let client_id = self
                        .next_client_id
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let clients = self.clients.clone();

                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, client_id, clients).await {
                            error!("Error handling connection {}: {}", client_id, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    pub async fn broadcast(&self, message: &str) {
        let mut disconnected = Vec::new();

        for entry in self.clients.iter() {
            let client_id = *entry.key();
            let sender = entry.value();

            if sender.send(message.to_string()).is_err() {
                disconnected.push(client_id);
            }
        }

        for client_id in disconnected {
            self.clients.remove(&client_id);
            info!("Removed disconnected client: {}", client_id);
        }
    }
}

async fn handle_connection(
    stream: TcpStream,
    client_id: ClientId,
    clients: Arc<DashMap<ClientId, mpsc::UnboundedSender<String>>>,
) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    clients.insert(client_id, tx);

    info!("Client {} registered", client_id);

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received from client {}: {}", client_id, text);
                }
                Ok(Message::Close(_)) => {
                    info!("Client {} closed connection", client_id);
                    break;
                }
                Err(e) => {
                    warn!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    clients.remove(&client_id);
    info!("Client {} disconnected", client_id);

    Ok(())
}