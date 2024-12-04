use crate::uuid::Uuid;
use serde::Serialize;


#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    username: String,
    email_address: String,
}

impl User {
    pub fn new(id: Uuid, username: String, email_address: String) -> Self {
        User {
            id,
            username,
            email_address,
        }
    }
}
