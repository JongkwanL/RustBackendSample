use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub backup_interval_sec: u64,
    pub backup_target_path: String,
    // 필요한 설정 필드를 자유롭게 추가
}

pub fn load_config() -> Result<AppConfig> {
    // 예: `config.json` 파일에서 설정을 읽어온다고 가정
    // 파일 경로는 상황에 따라 다를 수 있음
    let contents = fs::read_to_string("config.json")?;
    let config: AppConfig = serde_json::from_str(&contents)?;
    Ok(config)
}
