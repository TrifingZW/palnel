use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub auth: AuthConfig,
    pub server: ServerConfig,
    pub palworld: PalworldConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub http_port: u16,
    pub https_port: u16,
    pub tls: Option<ServerTlsConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerTlsConfig {
    pub enabled: bool,
    pub cert: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PalworldConfig {
    pub executable: String,
    pub workspace: String,
    pub args: Vec<String>,
    pub ip: String,
    pub rest_port: u16,
    pub username: String,
    pub password: String,
}

impl Default for PalworldConfig {
    fn default() -> Self {
        Self {
            executable: String::new(),
            workspace: String::new(),
            args: Vec::new(),
            ip: "127.0.0.1".to_string(),
            rest_port: 8212,
            username: "admin".to_string(),
            password: String::new(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auth: AuthConfig {
                username: "admin".to_string(),
                password: "123456".to_string(),
            },
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                http_port: 80,
                https_port: 443,
                tls: Some(ServerTlsConfig {
                    enabled: false,
                    cert: "".to_string(),
                    key: "".to_string(),
                }),
            },
            palworld: PalworldConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn load() -> anyhow::Result<AppConfig> {
        info!("Loading config.toml");

        let config_path = "config.toml";

        if !Path::new(config_path).exists() {
            let default_config = AppConfig::default();
            let toml_string = toml::to_string(&default_config)?;
            fs::write(config_path, toml_string)?;
            info!("未找到配置文件，已自动生成默认的 config.toml");
            return Ok(default_config);
        }

        let config_content = fs::read_to_string(config_path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}
