use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub download_dir: String,
    pub max_connections: u32,
    pub split_count: u32,
    pub min_split_size: String,
    pub user_agent: String,
    pub auto_start_aria2: bool,
    pub theme: String,
    pub max_overall_download_limit_kb_per_sec: u64,
    pub max_download_limit_kb_per_sec: u64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_dir: "downloads".to_string(),
            max_connections: 16,
            split_count: 16,
            min_split_size: "1M".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            auto_start_aria2: true,
            theme: "system".to_string(),
            max_overall_download_limit_kb_per_sec: 0,
            max_download_limit_kb_per_sec: 0,
        }
    }
}

impl AppSettings {
    pub fn format_bandwidth(kb_per_sec: u64) -> String {
        if kb_per_sec == 0 {
            "Unlimited".to_string()
        } else if kb_per_sec >= 1024 {
            format!("{:.2} MB/s", kb_per_sec as f64 / 1024.0)
        } else {
            format!("{} KB/s", kb_per_sec)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistoryItem {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub size: u64,
    pub status: String,
    pub downloaded_at: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DownloadHistory {
    pub items: Vec<DownloadHistoryItem>,
}
