use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

mod config;
mod daemon;
mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter("info")  // RUST_LOG=debug 처럼 환경변수로 레벨 제어 가능
        .init();

    // 2. 설정 불러오기
    let cfg = match config::load_config() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load config: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Starting my_rust_daemon...");

    // 3. 데몬 로직 실행
    if let Err(e) = daemon::run_daemon(cfg).await {
        error!("Daemon failed: {:?}", e);
        std::process::exit(1);
    }

    info!("Daemon exited gracefully.");
    Ok(())
}
