use shuttle_persist::PersistInstance;

use crate::domain::{Pona, PonaRegistry};

pub struct ShuttlePersistPonaRegistry {
    pub persist: PersistInstance,
}

const PONA_PREFIX: &str = "pona_";

impl PonaRegistry for ShuttlePersistPonaRegistry {
    fn get_all_for_lili(&self, lili_id: &str) -> Vec<Pona> {
        self.persist
            .list()
            .expect("should be instantiaed")
            .iter()
            .filter(|key| key.starts_with(PONA_PREFIX))
            .map(|key| self.persist.load::<Pona>(&key))
            .flatten()
            .collect::<Vec<Pona>>()
            .into_iter()
            .filter(|pona| pona.lili_id == lili_id)
            .collect()
    }

    fn get_all_for_username(&self, username_id: &str) -> Vec<Pona> {
        self.persist
            .list()
            .expect("should be instantiaed")
            .iter()
            .filter(|key| key.starts_with(PONA_PREFIX))
            .map(|key| self.persist.load::<Pona>(&key))
            .flatten()
            .collect::<Vec<Pona>>()
            .into_iter()
            .filter(|pona| pona.username_id == username_id)
            .collect()
    }

    fn create_pona(&self, pona: Pona) -> Result<(), String> {
        let result = self.persist.save(
            format!("{}{}_{}", PONA_PREFIX, pona.username_id, pona.lili_id).as_str(),
            &pona,
        );

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error saving pona".to_string()),
        }
    }

    fn delete_pona(&self, pona: Pona) -> Result<(), String> {
        let result = self.persist.remove(
            format!("{}{}_{}", PONA_PREFIX, pona.username_id, pona.lili_id).as_str(),
        );

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error deleting pona".to_string()),
        }
    }
}
