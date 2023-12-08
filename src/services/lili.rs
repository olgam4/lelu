use crate::domain::{Lili, LiliRegistryRef, PonaRegistryRef, Profile, ProfileRegistryRef};

pub struct LiliService {
    lili_registry: LiliRegistryRef,
    profile_registry: ProfileRegistryRef,
    pona_registry: PonaRegistryRef,
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
    pub fn new(
        lili_registry: LiliRegistryRef,
        profile_registry: ProfileRegistryRef,
        pona_registry: PonaRegistryRef,
    ) -> Self {
        Self {
            lili_registry,
            profile_registry,
            pona_registry,
        }
    }

    pub fn get_all_lilis(&self, username: &str) -> Vec<Lili> {
        let all_lilis = self.lili_registry.get_all();

        let liked_by_me = self
            .pona_registry
            .get_all_for_username(username)
            .into_iter()
            .map(|pona| pona.lili_id)
            .collect::<Vec<String>>();

        dbg!(&liked_by_me);

        let all_lilis = all_lilis
            .into_iter()
            .map(|mut lili| {
                let likes = self
                    .pona_registry
                    .get_all_for_lili(&lili.id)
                    .into_iter()
                    .map(|pona| pona.username_id)
                    .collect::<Vec<String>>();

                lili.ponas = likes.len() as i64;

                lili.liked_by_me = liked_by_me.contains(&lili.id);
                lili
            });

        all_lilis.collect()
    }

    pub fn get_all_lilis_from_user(&self, username: &str) -> Result<Vec<LiliByProfile>, String> {
        let lilis: Vec<Lili> = self
            .lili_registry
            .get_all()
            .into_iter()
            .filter(|lili| lili.username == username)
            .collect();

        let liked_by_me = self
            .pona_registry
            .get_all_for_username(username)
            .into_iter()
            .map(|pona| pona.lili_id)
            .collect::<Vec<String>>();
        let lilis = lilis
            .into_iter()
            .map(|mut lili| {
                lili.liked_by_me = liked_by_me.contains(&lili.id);
                lili
            })
            .collect::<Vec<Lili>>();

        let lilis = lilis
            .into_iter()
            .map(|mut lili| {
                let likes = self
                    .pona_registry
                    .get_all_for_lili(&lili.id)
                    .into_iter()
                    .map(|pona| pona.username_id)
                    .collect::<Vec<String>>();

                lili.ponas = likes.len() as i64;
                lili
            })
            .collect::<Vec<Lili>>();

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
            liked_by_me: false,
            ponas: 0,
        };

        self.lili_registry.create_lili(lili.clone())?;

        Ok((lili, profile))
    }

    pub fn pona_lili(&self, lili_id: &str, username: &str) -> Result<(), String> {
        println!("pona_lili_from_service");
        dbg!(&lili_id);

        let lili = self
            .lili_registry
            .get_all()
            .into_iter()
            .filter(|lili| lili.id == lili_id)
            .next()
            .ok_or("Lili not found".to_string())?;

        println!("pona_lili_after_lili");
        dbg!(&lili);

        let pona = crate::domain::Pona {
            lili_id: lili.id.clone(),
            username_id: username.to_string(),
        };

        println!("pona_lili_after_pona");
        dbg!(&pona);

        self.pona_registry.create_pona(pona)?;

        Ok(())
    }

    pub fn ike_lili(&self, lili_id: &str, username: &str) -> Result<(), String> {
        let lili = self
            .lili_registry
            .get_all()
            .into_iter()
            .filter(|lili| lili.id == lili_id)
            .next()
            .ok_or("Lili not found".to_string())?;

        let pona = crate::domain::Pona {
            lili_id: lili.id.clone(),
            username_id: username.to_string(),
        };

        self.pona_registry.delete_pona(pona)?;

        Ok(())
    }
}
