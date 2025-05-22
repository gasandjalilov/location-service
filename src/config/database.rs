use crate::config::configs::DatabaseConfig;
use mongodm::mongo::options::ClientOptions;
use mongodm::mongo::{Client, Database};

pub struct MNGDatabase {
    pub client: Client,
    pub db: Database,
}

impl MNGDatabase {
    pub async fn new(config: &DatabaseConfig) -> anyhow::Result<Self> {
        let client_options = ClientOptions::parse(&config.uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&config.database);
        Ok(Self { client, db })
    }
}
