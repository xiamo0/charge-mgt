//! 网关服务入口：加载配置、初始化应用并启动 WebSocket 服务

use charge_mgt_gateway::app::Application;
use charge_mgt_gateway::config::Config;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("正在加载配置...");
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("配置加载失败: {}", e);
            std::process::exit(1);
        }
    };

    info!("正在初始化应用...");
    let app = match Application::new(config).await {
        Ok(app) => app,
        Err(e) => {
            error!("应用创建失败: {}", e);
            std::process::exit(1);
        }
    };

    info!("正在启动网关...");
    if let Err(e) = app.run().await {
        error!("应用运行错误: {}", e);
        std::process::exit(1);
    }
}
