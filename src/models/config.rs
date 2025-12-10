//! Configuration model loaded from external sources.

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
/// Basic configuration shared across handlers.
pub struct ServerConfig {
    pub domain: String,
    pub address: String,
    pub port: u16,
    pub auth_service_url: String,
    pub templates_dir: String,
    pub secret: String,
    pub upload_path: String,
}
