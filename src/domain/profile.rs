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
