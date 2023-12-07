use crate::domain::{Profile, ProfileRegistryRef};

pub struct ProfileService {
    pub profile_registry: ProfileRegistryRef,
}

impl ProfileService {
    pub fn new(profile_registry: ProfileRegistryRef) -> Self {
        Self { profile_registry }
    }

    pub fn get_profile(&self, username: &str) -> Option<Profile> {
        self.profile_registry.get_profile(username)
    }

    pub fn create_profile(&self, profile: Profile) -> Result<(), String> {
        self.profile_registry.create_profile(profile)
    }
}
