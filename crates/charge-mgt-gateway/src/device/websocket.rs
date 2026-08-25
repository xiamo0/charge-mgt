//! WebSocket 服务器
//!
//! 监听 TCP 端口，接受充电桩 WebSocket 连接，
//! 读写分离：读任务处理上行消息，写任务发送响应。
//!
//! 安全（OCPP 1.6）：
//! - 模式 1/2：明文 ws://
//! - 模式 3/4：wss://（rustls TLS）
//! - 模式 5/6：wss:// + mTLS（CA 校验客户端证书，启用自动由 app.rs 注入）
//! - 模式 2/4/6：WS 升级时调 cloud /internal/auth/verify 验证密码（fail-closed）

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, ReadHalf, WriteHalf};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::accept_hdr_async;
use tracing::{error, info, warn};

use crate::cloud::{ConnectionManager, KafkaProducer};
use crate::config::{CloudConfig, DeviceConfig};
use crate::error::{GatewayError, Result};
use crate::response_channel::ResponseChannel;
use crate::security::basic_auth::verify_via_cloud;
use crate::security::policy::SecurityMode;

/// 类型别名：能同时作为读写流的 trait object
trait AsyncReadWrite: tokio::io::AsyncRead + tokio::io::AsyncWrite {}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite> AsyncReadWrite for T {}

/// WebSocket 服务端，管理充电桩接入
pub struct WebSocketServer {
    config: DeviceConfig,
    /// 安全模式（决定 TLS 是否启用）
    security_mode: SecurityMode,
    /// 可选 TLS acceptor（仅模式 3/4）
    tls_acceptor: Option<TlsAcceptor>,
    /// 云端配置（用于 Basic Auth 验证）
    cloud: CloudConfig,
    /// HTTP 客户端（Basic Auth 验证复用）
    http_client: reqwest::Client,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    response_channel: Arc<dyn ResponseChannel>,
    gateway_id: String,
    gateway_host: String,
}

impl WebSocketServer {
    pub fn new(
        config: DeviceConfig,
        security_mode: SecurityMode,
        tls_acceptor: Option<TlsAcceptor>,
        cloud: CloudConfig,
        http_client: reqwest::Client,
        connection_manager: Arc<ConnectionManager>,
        kafka_producer: Arc<KafkaProducer>,
        response_channel: Arc<dyn ResponseChannel>,
        gateway_id: String,
        gateway_host: String,
    ) -> Self {
        Self {
            config,
            security_mode,
            tls_acceptor,
            cloud,
            http_client,
            connection_manager,
            kafka_producer,
            response_channel,
            gateway_id,
            gateway_host,
        }
    }

    /// 绑定监听地址，循环接受连接并为每个连接 spawn 独立任务
    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.listen_addr, self.config.listen_port);
        let addr: SocketAddr = addr
            .parse()
            .map_err(|e| GatewayError::Config(format!("无效地址: {}", e)))?;

        info!(
            "OCPP WebSocket 监听 {}://{}（安全模式: {:?}）",
            self.security_mode.scheme(),
            addr,
            self.security_mode
        );

        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| GatewayError::WebSocket(format!("绑定失败: {}", e)))?;

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    info!("新连接来自: {}", peer_addr);
                    let connection_manager = self.connection_manager.clone();
                    let kafka_producer = self.kafka_producer.clone();
                    let response_channel = self.response_channel.clone();
                    let gateway_id = self.gateway_id.clone();
                    let gateway_host = self.gateway_host.clone();
                    let tls_acceptor = self.tls_acceptor.clone();
                    let security_mode = self.security_mode;
                    let cloud = self.cloud.clone();
                    let http_client = self.http_client.clone();
                    tokio::spawn(handle_connection(
                        stream,
                        peer_addr,
                        security_mode,
                        tls_acceptor,
                        cloud,
                        http_client,
                        connection_manager,
                        kafka_producer,
                        response_channel,
                        gateway_id,
                        gateway_host,
                    ));
                }
                Err(e) => {
                    error!("接受连接失败: {}", e);
                }
            }
        }
    }
}

/// 处理单条 WebSocket 连接的生命周期（TLS 握手 → WS 升级 → 业务消息）
async fn handle_connection(
    stream: tokio::net::TcpStream,
    peer_addr: SocketAddr,
    security_mode: SecurityMode,
    tls_acceptor: Option<TlsAcceptor>,
    cloud: CloudConfig,
    http_client: reqwest::Client,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    response_channel: Arc<dyn ResponseChannel>,
    gateway_id: String,
    gateway_host: String,
) {
    // 1. （可选）TLS 握手 + WS 升级
    // MaybeTlsStream::Rustls 在 tokio-tungstenite 0.21 的 __rustls-tls 私有 feature 后，
    // 用 tokio::io::split 拆出 TLS 流读写半边，再通过 `Box<dyn AsyncReadWrite>` 抹平
    // 为单一具体类型，让 accept 返回统一的 WebSocketStream<Box<...>>。
    //
    // 用 accept_hdr_async 拦截升级请求头，捕获 Authorization（Basic Auth 模式 2/4）
    let captured_auth = Arc::new(std::sync::Mutex::new(None::<String>));
    let auth_required = security_mode.requires_basic();

    let ws_stream: WebSocketStream<Box<dyn AsyncReadWrite + Unpin + Send>> = match tls_acceptor {
        Some(acceptor) => {
            let tls = match acceptor.accept(stream).await {
                Ok(t) => t,
                Err(e) => {
                    warn!("TLS 握手失败（来自 {}）: {}", peer_addr, e);
                    return;
                }
            };
            let (r, w) = tokio::io::split(tls);
            let s: Box<dyn AsyncReadWrite + Unpin + Send> = Box::new(SplitStream(r, w));
            match accept_with_auth_capture(s, auth_required, captured_auth.clone()).await {
                Ok(ws) => ws,
                Err(e) => {
                    warn!("WebSocket 升级失败（TLS 模式）, 来自 {}: {}", peer_addr, e);
                    return;
                }
            }
        }
        None => {
            let s: Box<dyn AsyncReadWrite + Unpin + Send> = Box::new(stream);
            match accept_with_auth_capture(s, auth_required, captured_auth.clone()).await {
                Ok(ws) => ws,
                Err(e) => {
                    warn!("WebSocket 升级失败，来自 {}: {}", peer_addr, e);
                    return;
                }
            }
        }
    };

    // 2. Basic Auth 密码验证（模式 2/4）
    // fail-closed：cloud 不可达或密码错 → 关闭连接。
    if security_mode.requires_basic() {
        let auth_header = captured_auth.lock().unwrap().clone();
        let Some((identity, password)) = auth_header
            .as_deref()
            .and_then(crate::security::auth::parse_basic_auth)
        else {
            warn!(
                "模式 {:?} 要求 Basic Auth 但 Authorization 头缺失或格式错，来自 {}",
                security_mode, peer_addr
            );
            return;
        };

        match verify_via_cloud(&cloud, &http_client, &identity, &password).await {
            Ok(true) => info!("桩 {} Basic Auth 通过（来自 {}）", identity, peer_addr),
            Ok(false) => {
                warn!(
                    "桩 {} Basic Auth 失败（密码错或不存在），来自 {}",
                    identity, peer_addr
                );
                return;
            }
            Err(e) => {
                warn!(
                    "桩 {} Basic Auth 验证失败（fail-closed）: {}",
                    identity, e
                );
                return;
            }
        }
    }

    let (ws_write, ws_read) = ws_stream.split();

    let (response_tx, mut response_rx) = mpsc::unbounded_channel();

    let mut connection = crate::device::connection::Connection::new(
        peer_addr,
        connection_manager.clone(),
        kafka_producer,
        response_channel,
        gateway_id,
        gateway_host,
        response_tx.clone(),
    );

    let mut ws_read = ws_read;
    let mut ws_write = ws_write;

    let read_task = async {
        use tokio_tungstenite::tungstenite::Message;

        while let Some(msg) = ws_read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("收到消息，来自 {}: {}", peer_addr, text);
                    match connection.handle_message(&text).await {
                        Ok(()) => {}
                        Err(e) => {
                            error!("处理消息失败，来自 {}: {}", peer_addr, e);
                            let error_response =
                                connection.create_call_error("", "InternalError", &e.to_string());
                            response_tx.send(error_response).ok();
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("连接已关闭: {}", peer_addr);
                    connection.on_disconnect().await;
                    break;
                }
                Err(e) => {
                    warn!("读取消息失败，来自 {}: {}", peer_addr, e);
                    connection.on_disconnect().await;
                    break;
                }
                _ => {}
            }
        }
    };

    let write_task = async {
        use tokio_tungstenite::tungstenite::Message;

        while let Some(response) = response_rx.recv().await {
            if ws_write.send(Message::Text(response)).await.is_err() {
                warn!("发送消息失败，目标 {}: WebSocket 写入错误", peer_addr);
                break;
            }
        }
    };

    tokio::select! {
        _ = read_task => {},
        _ = write_task => {},
    }

    connection.on_disconnect().await;
}

/// 包装 `tokio::io::split(tls)` 的两端，使 `accept_async` 能直接消费
struct SplitStream<R, W>(ReadHalf<R>, WriteHalf<W>);

impl<R, W> AsyncRead for SplitStream<R, W>
where
    R: AsyncRead + Unpin,
    W: Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl<R, W> AsyncWrite for SplitStream<R, W>
where
    R: Unpin,
    W: AsyncWrite + Unpin,
{
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.1).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.1).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.1).poll_shutdown(cx)
    }
}

/// WS 升级：拦截 Authorization 头并捕获（Basic Auth 模式）
async fn accept_with_auth_capture<S>(
    stream: S,
    auth_required: bool,
    captured: Arc<std::sync::Mutex<Option<String>>>,
) -> std::result::Result<WebSocketStream<S>, GatewayError>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};

    let ws = accept_hdr_async(stream, move |req: &Request, response: Response| {
        if auth_required {
            if let Some(auth) = req
                .headers()
                .get("authorization")
                .and_then(|v| v.to_str().ok())
            {
                *captured.lock().unwrap() = Some(auth.to_string());
            }
        }
        Ok::<_, ErrorResponse>(response)
    })
    .await
    .map_err(|e| GatewayError::WebSocket(format!("WS 升级失败: {e}")))?;

    Ok(ws)
}