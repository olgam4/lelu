use shuttle_persist::PersistInstance;

use crate::domain::{User, UserRegistry};

pub struct ShuttlePersistUserRegistry {
    pub persist: PersistInstance,
}

const USER_PREFIX: &str = "user_";

impl UserRegistry for ShuttlePersistUserRegistry {
    fn get_user(&self, username: &str) -> Option<User> {
        let list = self.persist.list().expect("should be instantiaed");

        list.iter()
            .filter(|key| key.starts_with(USER_PREFIX))
            .map(|key| self.persist.load::<User>(&key).unwrap())
            .collect::<Vec<User>>()
            .into_iter()
            .filter(|user| user.username == username)
            .next()
    }

    fn create_user(&self, user: User) -> Result<(), String> {
        let result = self
            .persist
            .save(format!("{}{}", USER_PREFIX, user.username).as_str(), &user);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error saving user".to_string()),
        }
    }
}
