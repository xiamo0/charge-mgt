use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};

use crate::cloud::{ConnectionManager, KafkaProducer};
use crate::config::DeviceConfig;
use crate::error::{GatewayError, Result};

pub struct WebSocketServer {
    config: DeviceConfig,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    gateway_id: String,
    gateway_host: String,
}

impl WebSocketServer {
    pub fn new(
        config: DeviceConfig,
        connection_manager: Arc<ConnectionManager>,
        kafka_producer: Arc<KafkaProducer>,
        gateway_id: String,
        gateway_host: String,
    ) -> Self {
        Self {
            config,
            connection_manager,
            kafka_producer,
            gateway_id,
            gateway_host,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.listen_addr, self.config.listen_port);
        let addr: SocketAddr = addr
            .parse()
            .map_err(|e| GatewayError::Config(format!("Invalid address: {}", e)))?;

        info!("WebSocket server starting on {}", addr);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| GatewayError::WebSocket(format!("Failed to bind: {}", e)))?;

        info!("WebSocket server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    info!("New connection from: {}", peer_addr);
                    let connection_manager = self.connection_manager.clone();
                    let kafka_producer = self.kafka_producer.clone();
                    let gateway_id = self.gateway_id.clone();
                    let gateway_host = self.gateway_host.clone();
                    tokio::spawn(handle_connection(
                        stream,
                        peer_addr,
                        connection_manager,
                        kafka_producer,
                        gateway_id,
                        gateway_host,
                    ));
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    peer_addr: SocketAddr,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    gateway_id: String,
    gateway_host: String,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            warn!("WebSocket handshake failed from {}: {}", peer_addr, e);
            return;
        }
    };

    let (write, read) = ws_stream.split();

    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::Message;

    let mut connection = crate::device::connection::Connection::new(
        peer_addr,
        connection_manager,
        kafka_producer,
        gateway_id,
        gateway_host,
    );

    let mut read = read;
    let mut write = write;
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received from {}: {}", peer_addr, text);
                match connection.handle_message(&text).await {
                    Ok(Some(response)) => {
                        if let Err(e) = write.send(Message::Text(response)).await {
                            error!("Failed to send response to {}: {}", peer_addr, e);
                            break;
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        error!("Error handling message from {}: {}", peer_addr, e);
                        let error_response =
                            serde_json::json!([4, "", "InternalError", e.to_string(), {}])
                                .to_string();
                        if let Err(e) = write.send(Message::Text(error_response)).await {
                            error!("Failed to send error response to {}: {}", peer_addr, e);
                            break;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("Connection closed: {}", peer_addr);
                connection.on_disconnect().await;
                break;
            }
            Err(e) => {
                warn!("Error reading from {}: {}", peer_addr, e);
                connection.on_disconnect().await;
                break;
            }
            _ => {}
        }
    }
}
