use anyhow::Result;
use tracing::info;

use crate::config::AppConfig;

pub async fn perform_backup_task(cfg: &AppConfig) -> Result<()> {
    // 실제 백업 로직. 여기선 단순히 로그만 남긴다고 가정
    info!("Starting backup task. Target path: {}", cfg.backup_target_path);

    // 실제 작업 예시 (파일 복사, 압축, 업로드 등)
    // tokio::fs::copy("source_path", &cfg.backup_target_path).await?;

    // 간단한 예시를 위해, 잠시 대기
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    info!("Backup task finished!");
    Ok(())
}
