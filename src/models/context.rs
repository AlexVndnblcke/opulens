use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};
use tracing::{event, Level};

use crate::uuid::Uuid;

use super::{upload::Upload, user::User};

#[derive(Default, Debug)]
pub struct Context {
    users: RwLock<HashMap<Uuid, Arc<Mutex<User>>>>,
    uploads: RwLock<Vec<Arc<Mutex<Upload>>>>,
}

impl Context {
    pub async fn get_user(&self, id: Uuid) -> Option<Arc<Mutex<User>>> {
        let users = self.users.read().await;
        match users.get(&id) {
            Some(mutex_user) => Some(mutex_user.clone()),
            None => None,
        }
    }

    pub async fn add_user(&mut self, user: User) {
        event!(Level::INFO, "adding user to storage with id {:?}", user.id);
        let mut users = self.users.write().await;
        users.insert(user.id.clone(), Arc::new(Mutex::new(user)));
        event!(Level::DEBUG, "number of users in storage: {}", users.len());
    }

    pub async fn add_upload(&mut self, upload: Upload) {
        event!(Level::INFO, "adding upload to storage for user with id {:?} and name {}", upload.user_id, upload.get_file_name());
        let mut uploads = self.uploads.write().await;
        uploads.push(Arc::new(Mutex::new(upload)));
        event!(Level::DEBUG, "number of files in storage: {}", uploads.len());
    }
}
