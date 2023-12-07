use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub username: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub website: String,
    pub location: String,
    pub birthday: String,
}

pub trait ProfileRegistry: Send + Sync {
    fn get_profile(&self, username: &str) -> Option<Profile>;
    fn create_profile(&self, profile: Profile) -> Result<(), String>;
}

pub type ProfileRegistryRef = Arc<dyn ProfileRegistry>;
