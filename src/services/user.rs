use crate::domain::{User, UserRegistryRef};

pub struct UserService {
    pub user_registry: UserRegistryRef,
}

impl UserService {
    pub fn new(user_registry: UserRegistryRef) -> Self {
        Self { user_registry }
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        self.user_registry.get_user(username)
    }

    pub fn create_user(&self, user: User) -> Result<(), String> {
        self.user_registry.create_user(user)
    }
}
