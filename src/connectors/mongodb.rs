use bson::doc;
use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::Repository;

#[derive(Debug)]
pub enum RepositoryError {
    NoDefaultDatabase,
}

#[derive(Debug)]
pub struct MongoRepository<T>
where
    T: Send + Sync,
{
    client: Client,
    database: Database,
    collection: Collection<T>,
}

impl<T> MongoRepository<T>
where
    T: Send + Sync,
{
    /// Open a Mongo repository from the default database and the collection given by its name.
    ///
    /// If database_name is given, this overrides taking the default database (the default database
    /// can be included in the connection_uri).
    pub fn open(
        client: Client,
        database_name: Option<&str>,
        collection_name: &str,
    ) -> Result<Self, RepositoryError> {
        let db = if let Some(db_name) = database_name {
            client.database(db_name)
        } else if let Some(db) = client.default_database() {
            db
        } else {
            return Err(RepositoryError::NoDefaultDatabase);
        };

        let collection = db.collection(collection_name);

        Ok(Self {
            client,
            database: db,
            collection,
        })
    }

    pub fn collection(&self, name: &str) -> Collection<T> {
        self.database.collection::<T>(name)
    }
}

impl<T> Repository<T> for MongoRepository<T>
where
    T: Clone + Debug + Sync + Send + Serialize + for<'a> Deserialize<'a>,
{
    type Id = bson::Uuid;
    type E = mongodb::error::Error;

    async fn update(&mut self, id: &Self::Id, entity: &T) -> Result<(), Self::E> {
        let result = self
            .collection
            .replace_one(
                doc! {
                    "_id": id,
                },
                entity,
            )
            .await;

        result.map(|_| Ok(()))?
    }

    async fn create(&mut self, entity: &T) -> Result<(), Self::E> {
        let create_result = self.collection.insert_one(entity).await;
        create_result.map(|_| Ok(()))?
    }

    async fn load(&self, id: &Self::Id) -> Result<Option<T>, Self::E> {
        self.collection
            .find_one(doc! {
                "_id": id,
            })
            .await
    }
}
