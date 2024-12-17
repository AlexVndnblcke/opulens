use log::{error, info};
use mongodb::Client;
use tracing::{event, Level};

use crate::config::Config;
use crate::connectors::mongodb::{MongoRepository, RepositoryError};
use crate::connectors::Repository;
use crate::models::user::MongoUser;
use crate::service::Identify;
use crate::uuid::Uuid;
use crate::{models::user::User, service::DataAccessService};

#[derive(Debug)]

pub enum MongoDBError {
    MongoClientError(String),
    RepositoryError(RepositoryError),
}

#[derive(Debug)]
pub struct MongoDBService {
    client: Client,
    user_repo: MongoRepository<MongoUser>,
}

impl MongoDBService {
    pub async fn try_from_config(config: &Config) -> Result<Self, MongoDBError> {
        let client = match Client::with_uri_str(&config.mongodb.connection_uri).await {
            Ok(client) => client,
            Err(e) => return Err(MongoDBError::MongoClientError(e.to_string())),
        };
        let user_repo = match MongoRepository::<MongoUser>::open(client.clone(), None, "users") {
            Ok(repo) => repo,
            Err(e) => return Err(MongoDBError::RepositoryError(e)),
        };

        Ok(MongoDBService::new(client, user_repo))
    }

    fn new(client: Client, user_repo: MongoRepository<MongoUser>) -> Self {
        Self { client, user_repo }
    }
}

impl DataAccessService<User> for MongoDBService {
    type E = String;

    async fn create(&mut self, user: &User) -> Result<(), Self::E> {
        info!("adding user to storage with id {:?}", user.id());

        let mongo_user: MongoUser = user.clone().into();
        let create_result = self.user_repo.create(&mongo_user).await;
        if let Err(e) = create_result {
            error!("create entity in mongodb failed: {:?}", e);
            return Err("failed to save user in database".to_string());
        }

        event!(Level::DEBUG, "user with id: {:?}", mongo_user._id);
        Ok(())
    }

    async fn update(&mut self, user: &User) -> Result<(), Self::E> {
        info!("updating user with id {:?}", user.id());

        let mongo_user: MongoUser = user.clone().into();
        let update_result = self.user_repo.update(&mongo_user._id, &mongo_user).await;
        if let Err(e) = update_result {
            error!("update entity in mongodb failed: {:?}", e);
            return Err("failed to safe user update in database".to_string());
        }
        Ok(())
    }

    async fn read(&self, id: &Uuid) -> Result<Option<User>, Self::E> {
        info!("retrieving user for id {:?}", id);

        let mongo_user = self.user_repo.load(&id.clone().into()).await;
        mongo_user
            .map(|o| o.map(|u| u.into()))
            .map_err(|e| e.to_string())
    }
}
