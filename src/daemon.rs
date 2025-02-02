use anyhow::Result;
use tracing::{info, error};
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio_stream::StreamExt;

use signal_hook::consts::SIGTERM;
use signal_hook_tokio::Signals;

use crate::config::AppConfig;
use crate::tasks::backup_task::perform_backup_task;

pub async fn run_daemon(cfg: AppConfig) -> Result<()> {
    // 1. Unix 신호 핸들러 세팅
    let mut signals = Signals::new(&[SIGTERM])?;

    // 2. 주기적 백업 작업을 위한 interval
    let backup_interval = Duration::from_secs(cfg.backup_interval_sec);

    info!("Daemon running, backup interval: {:?} seconds", backup_interval);

    loop {
        select! {
            // 다음 신호를 대기
            maybe_signal = signals.next() => {
                if let Some(signal) = maybe_signal {
                    info!("Received signal: {}", signal);
                    if signal == SIGTERM {
                        info!("Received SIGTERM signal. Stopping daemon...");
                        break;
                    }
                } else {
                    // 신호 스트림이 예기치 않게 종료된 경우
                    error!("Signal stream ended unexpectedly. Exiting daemon...");
                    break;
                }
            },

            // 주기적인 백업 작업 수행
            _ = sleep(backup_interval) => {
                if let Err(e) = perform_backup_task(&cfg).await {
                    error!("Backup task failed: {:?}", e);
                }
            }
        }
    }

    info!("Daemon cleanup completed. Exiting...");
    Ok(())
}
