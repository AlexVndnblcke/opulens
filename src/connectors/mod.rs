pub mod mongodb;

pub trait Repository<Entity> {
    type Id;
    type E;

    async fn update(&mut self, id: &Self::Id, entity: &Entity) -> Result<(), Self::E>;
    async fn create(&mut self, entity: &Entity) -> Result<(), Self::E>;
    async fn load(&self, id: &Self::Id) -> Result<Option<Entity>, Self::E>;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Repository;

    #[derive(Clone, PartialEq, Debug)]
    struct MockEntity {
        id: u8,
        name: String,
    }

    #[derive(Default)]
    struct MockRepo {
        entities: HashMap<u8, MockEntity>,
    }

    impl Repository<MockEntity> for MockRepo {
        type Id = u8;

        type E = String;

        async fn create(&mut self, entity: &MockEntity) -> Result<(), Self::E> {
            let id = entity.id;
            if self.entities.contains_key(&id) {
                return Err(String::from("entity already exists"));
            }
            self.entities.insert(id, entity.clone());
            Ok(())
        }

        async fn update(&mut self, id: &Self::Id, entity: &MockEntity) -> Result<(), Self::E> {
            if !self.entities.contains_key(id) {
                return Err(String::from("entity doesn't exist, cannot update"));
            }
            self.entities.insert(*id, entity.clone());
            Ok(())
        }

        async fn load(&self, id: &Self::Id) -> Result<Option<MockEntity>, Self::E> {
            self.entities
                .get(id)
                .map(|v| Some(v.clone()))
                .ok_or(String::from("entity not found"))
        }
    }

    #[tokio::test]
    async fn test_repo() {
        let mut repo = MockRepo::default();

        let mock_entity = MockEntity {
            id: 1,
            name: String::from("mock entity 1"),
        };

        repo.create(&mock_entity)
            .await
            .expect("no reason the create would fail");

        let retrieved_entity = repo
            .load(&mock_entity.id)
            .await
            .expect("just stored, so must exist");

        assert_eq!(retrieved_entity.unwrap(), mock_entity);
    }

    #[tokio::test]
    async fn test_repo_duplicate_create() {
        let mut repo = MockRepo::default();

        let mock_entity = MockEntity {
            id: 1,
            name: String::from("mock entity 1"),
        };

        repo.create(&mock_entity)
            .await
            .expect("no reason the create would fail");
        repo.create(&mock_entity)
            .await
            .expect_err("create should fail since entity is added already");
    }

    #[tokio::test]
    async fn test_repo_update_missing() {
        let mut repo = MockRepo::default();

        let mock_entity = MockEntity {
            id: 1,
            name: String::from("mock entity 1"),
        };

        repo.update(&mock_entity.id, &mock_entity)
            .await
            .expect_err("update should fail since entity is not added yet");
    }
}
