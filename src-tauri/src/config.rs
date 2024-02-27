use config::{Config, File, FileFormat};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::write;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    path: String,
    first_run: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            path: "<unset>".to_string(),
            first_run: true,
        }
    }
}

impl AppConfig {
    fn load_config() -> Config {
        let mut config = Config::default();
        config
            .merge(File::from_str(
                r#"
            {
                "path": "<unset>",
                "first_run": true
            }
            "#,
                FileFormat::Json,
            ))
            .unwrap();

        if let Some(config_dir) = config_dir() {
            let file_path: PathBuf = config_dir.join("youtube-downloader").join("config.json");

            if file_path.exists() {
                config
                    .merge(File::with_name(&file_path.to_string_lossy()))
                    .unwrap();
            }
        }

        config
    }

    fn save_config(config: &AppConfig) {
        if let Ok(json) = serde_json::to_string(config) {
            if let Some(config_dir) = config_dir() {
                let app_folder_path = config_dir.join("youtube-downloader");
                let file_path: PathBuf = app_folder_path.join("config.json");
                if let Err(err) = std::fs::create_dir_all(&app_folder_path) {
                    eprintln!("Error creating app folder: {}", err);
                }
                if let Err(err) = write(&file_path, json) {
                    eprintln!("Error while writing config: {}", err);
                }
            } else {
                eprintln!("Error getting config directory");
            }
        } else {
            eprintln!("Error serializing AppConfig to JSON");
        }
    }
}

#[tauri::command]
pub fn read_config() -> AppConfig {
    let config = AppConfig::load_config();
    config.try_into().unwrap_or_default()
}

#[tauri::command]
pub fn write_config(data: AppConfig) {
    let mut config = AppConfig::load_config();
    config.set("path", data.path).unwrap();
    config.set("first_run", data.first_run).unwrap();
    // more fields to set

    // Convert &Config to &AppConfig
    let app_config: &AppConfig = &config.try_into().unwrap_or_default();

    AppConfig::save_config(app_config);
}
