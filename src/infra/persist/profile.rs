use shuttle_persist::PersistInstance;

use crate::domain::{Profile, ProfileRegistry};

pub struct ShuttlePersistProfileRegistry {
    pub persist: PersistInstance,
}

const PROFILE_PREFIX: &str = "profile_";

impl ProfileRegistry for ShuttlePersistProfileRegistry {
    fn get_profile(&self, username: &str) -> Option<Profile> {
        let list = self.persist.list().expect("should be instantiaed");

        list.iter()
            .filter(|key| key.starts_with(PROFILE_PREFIX))
            .map(|key| self.persist.load::<Profile>(&key).unwrap())
            .collect::<Vec<Profile>>()
            .into_iter()
            .filter(|profile| profile.username == username)
            .next()
    }

    fn create_profile(&self, profile: Profile) -> Result<(), String> {
        self.persist
            .save(format!("{}{}", PROFILE_PREFIX, profile.username).as_str(), &profile)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
