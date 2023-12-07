use crate::domain::{Lili, LiliRegistryRef, Profile, ProfileRegistryRef};

pub struct LiliService {
    lili_registry: LiliRegistryRef,
    profile_registry: ProfileRegistryRef,
}

pub struct NewLili {
    pub text: String,
    pub from_username: String,
}

pub struct LiliByProfile {
    pub lili: Lili,
    pub profile: Profile,
}

impl LiliService {
    pub fn new(lili_registry: LiliRegistryRef, profile_registry: ProfileRegistryRef) -> Self {
        Self {
            lili_registry,
            profile_registry,
        }
    }

    pub fn get_all_lilis(&self) -> Vec<Lili> {
        self.lili_registry.get_all()
    }

    pub fn get_all_lilis_from_user(&self, username: &str) -> Result<Vec<LiliByProfile>, String> {
        let lilis: Vec<Lili> = self
            .lili_registry
            .get_all()
            .into_iter()
            .filter(|lili| lili.username == username)
            .collect();

        let profile = self
            .profile_registry
            .get_profile(username)
            .ok_or("Profile not found".to_string())?;

        Ok(lilis
            .into_iter()
            .map(|lili| LiliByProfile {
                lili,
                profile: profile.clone(),
            })
            .collect())
    }

    pub fn create_lili(&self, new_lili: NewLili) -> Result<(Lili, Profile), String> {
        let profile = self
            .profile_registry
            .get_profile(&new_lili.from_username)
            .ok_or("Profile not found".to_string())?;

        let lili = Lili {
            id: nanoid::nanoid!(32),
            text: new_lili.text,
            username: profile.username.clone(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        self.lili_registry.create_lili(lili.clone())?;

        Ok((lili, profile))
    }
}
