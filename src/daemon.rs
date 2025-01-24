use anyhow::Result;
use tracing::{info, error};
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio_stream::StreamExt; // <-- 추가

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
            // 신호 스트림에서 다음 신호가 들어오면 처리
            maybe_signal = signals.next() => {
                if let Some(signal) = maybe_signal {
                    info!("Received signal: {}", signal);
                    if signal == SIGTERM {
                        info!("Received SIGTERM signal. Stopping daemon...");
                        break;
                    }
                } else {
                    // 스트림이 종료된 경우 (드물지만) 루프 탈출
                    error!("Signal stream ended unexpectedly. Exiting daemon...");
                    break;
                }
            }

            // 설정에 지정된 시간만큼 대기 후 백업 작업 수행
            _ = sleep(backup_interval) => {
                if let Err(e) = perform_backup_task(&cfg).await {
                    error!("Backup task failed: {:?}", e);
                }
            }
        }
    }

    // 종료 직전 정리 작업 (예: DB 연결 해제, 임시파일 삭제 등)
    info!("Daemon cleanup completed. Exiting...");
    Ok(())
}
