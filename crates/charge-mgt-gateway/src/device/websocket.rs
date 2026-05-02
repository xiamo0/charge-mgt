use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};

use crate::cloud::MockKafkaProducer;
use crate::config::DeviceConfig;
use crate::error::{GatewayError, Result};
use crate::state::AppState;

pub struct WebSocketServer {
    config: DeviceConfig,
    state: Arc<AppState>,
    kafka_producer: Arc<MockKafkaProducer>,
    gateway_id: String,
    gateway_host: String,
}

impl WebSocketServer {
    pub fn new(
        config: DeviceConfig,
        state: Arc<AppState>,
        kafka_producer: Arc<MockKafkaProducer>,
        gateway_id: String,
        gateway_host: String,
    ) -> Self {
        Self {
            config,
            state,
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
                    let state = self.state.clone();
                    let config = self.config.clone();
                    let kafka_producer = self.kafka_producer.clone();
                    let gateway_id = self.gateway_id.clone();
                    let gateway_host = self.gateway_host.clone();
                    tokio::spawn(handle_connection(
                        stream,
                        peer_addr,
                        state,
                        config,
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
    state: Arc<AppState>,
    config: DeviceConfig,
    kafka_producer: Arc<MockKafkaProducer>,
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

    let (mut write, mut read) = ws_stream.split();

    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::Message;

    let mut connection = crate::device::connection::Connection::new(
        peer_addr,
        state,
        config,
        kafka_producer,
        gateway_id,
        gateway_host,
    );

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
                            error!("Failed to send error response: {}", e);
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
