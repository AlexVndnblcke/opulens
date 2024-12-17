use crate::service::Identify;
use crate::uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    id: Uuid,
    username: String,
    email_address: String,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, username: String, email_address: String) -> Self {
        User {
            id,
            username,
            email_address,
            created_at: Utc::now(),
        }
    }
}

impl Identify<Uuid> for User {
    fn id(&self) -> Uuid {
        self.id.clone()
    }
}

#[cfg(feature = "mongodb")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MongoUser {
    pub _id: bson::Uuid,
    username: String,
    email_address: String,
    created_at: bson::DateTime,
}

#[cfg(feature = "mongodb")]
impl From<User> for MongoUser {
    fn from(value: User) -> Self {
        MongoUser {
            _id: value.id.into(),
            username: value.username,
            email_address: value.email_address,
            created_at: value.created_at.into(),
        }
    }
}

#[cfg(feature = "mongodb")]
impl From<MongoUser> for User {
    fn from(value: MongoUser) -> Self {
        let uuid: uuid::Uuid = uuid::Uuid::from_bytes(value._id.bytes());

        Self {
            id: Uuid(uuid),
            username: value.username,
            email_address: value.email_address,
            created_at: value.created_at.into(),
        }
    }
}

#[cfg(test)] // Ensures this module is only included in test builds
mod tests {
    use bson::doc;
    use chrono::TimeZone;

    use super::*; // Import items from the parent module

    fn test_user() -> User {
        User {
            id: Uuid::new_v4(),
            created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            username: String::from("mock_user"),
            email_address: String::from("mock@mock.com"),
        }
    }

    #[test]
    fn test_to_bson() {
        let user = test_user();
        let mongo_user: MongoUser = user.clone().into();

        let bson_doc = bson::to_bson(&mongo_user).expect("unable to convert user to bson");

        let document = bson_doc.as_document().expect("should be a document");

        let bson_id: bson::Uuid = user.id.into();
        let bson_id: bson::Bson = bson_id.into();
        assert_eq!(
            document.get("_id").expect("_id should exist in bson doc"),
            &bson_id
        );
        assert_eq!(
            document
                .get("username")
                .expect("username should exist in bson doc"),
            &bson::Bson::String("mock_user".into())
        );
        assert_eq!(
            document
                .get("email_address")
                .expect("email_address should exist in bson doc"),
            &bson::Bson::String("mock@mock.com".into())
        );
        assert_eq!(
            document
                .get("created_at")
                .expect("created_at should exist in bson doc"),
            &bson::DateTime::from_chrono(user.created_at).into()
        );
    }

    #[test]
    fn test_from_bson() {
        let user = test_user();
        let expected_user: MongoUser = user.clone().into();
        let document = doc! {
            "_id": bson::Uuid::from(user.clone().id.0),
            "username": "mock_user",
            "email_address": "mock@mock.com",
            "created_at": bson::DateTime::from_chrono(user.created_at),
        };

        println!("{:?}", user);

        let user: MongoUser = bson::from_document(document)
            .expect("document is manually created, so it is expected to be valid");

        assert_eq!(expected_user._id, user._id);
        assert_eq!(expected_user.username, user.username);
        assert_eq!(expected_user.email_address, user.email_address);
        assert_eq!(expected_user.created_at, user.created_at);
    }
}
