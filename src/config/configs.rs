use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub uri: String,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            uri: env::var("MONGO_URI")
                .unwrap_or_else(|_| "mongodb://192.168.1.111:27017".to_string()),
            database: env::var("MONGO_DATABASE").unwrap_or_else(|_| "test".to_string()),
            username: env::var("MONGO_USER").ok(),
            password: env::var("MONGO_PASSWORD").ok(),
        }
    }
}
