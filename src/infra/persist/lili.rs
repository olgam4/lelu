use shuttle_persist::PersistInstance;

use crate::domain::{Lili, LiliRegistry};

pub struct ShuttlePersistLiliRegistry {
    pub persist: PersistInstance,
}

const LILI_PREFIX: &str = "lili_";

impl LiliRegistry for ShuttlePersistLiliRegistry {
    fn get_all(&self) -> Vec<Lili> {
        let list = self.persist.list().expect("should be instantiaed");
        list.iter()
            .filter(|key| key.starts_with(LILI_PREFIX))
            .map(|key| self.persist.load::<Lili>(&key).unwrap())
            .collect::<Vec<Lili>>()
    }

    fn create_lili(&self, lili: Lili) -> Result<(), String> {
        let result = self
            .persist
            .save(format!("{}{}", LILI_PREFIX, lili.id).as_str(), &lili);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error saving lili".to_string()),
        }
    }
}
