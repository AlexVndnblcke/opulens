use crate::{models::user::User, uuid::Uuid};

pub mod mongodb;

pub trait Identify<Id> {
    fn id(&self) -> Id;
}

pub trait DataAccessService<Entity>
where
    Entity: Identify<Uuid>,
{
    type E;

    async fn create(&mut self, entity: &Entity) -> Result<(), Self::E>;
    async fn update(&mut self, user: &Entity) -> Result<(), Self::E>;
    async fn read(&self, id: &Uuid) -> Result<Option<User>, Self::E>;
}
