use charge_mgt_gateway::app::Application;
use charge_mgt_gateway::config::Config;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Loading configuration...");
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    info!("Initializing application...");
    let app = match Application::new(config) {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to create application: {}", e);
            std::process::exit(1);
        }
    };

    info!("Starting gateway...");
    if let Err(e) = app.run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}
