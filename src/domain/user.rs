use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

pub trait UserRegistry: Send + Sync {
    fn get_user(&self, username: &str) -> Option<User>;
    fn create_user(&self, user: User) -> Result<(), String>;
}

pub type UserRegistryRef = Arc<dyn UserRegistry>;
