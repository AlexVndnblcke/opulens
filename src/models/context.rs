use crate::config::Config;
use crate::connectors::mongodb::RepositoryError;
use crate::service::mongodb::{MongoDBError, MongoDBService};

#[derive(Debug)]
pub struct Context {
    #[cfg(feature = "mongodb")]
    pub data_service: MongoDBService,
}

impl Context {
    #[cfg(feature = "mongodb")]
    pub async fn try_from_config(config: &Config) -> Result<Self, MongoDBError> {
        let mongo_service = MongoDBService::try_from_config(config).await?;

        Ok(Self {
            data_service: mongo_service,
        })
    }
}

#[derive(Debug)]
pub enum ContextError {
    MongoClientError(String),
    RepositoryError(RepositoryError),
}
