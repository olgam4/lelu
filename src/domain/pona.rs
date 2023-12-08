use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pona {
    pub username_id: String,
    pub lili_id: String,
}

pub trait PonaRegistry {
    fn get_all_for_lili(&self, lili_id: &str) -> Vec<Pona>;
    fn get_all_for_username(&self, username_id: &str) -> Vec<Pona>;
    fn create_pona(&self, pona: Pona) -> Result<(), String>;
    fn delete_pona(&self, pona: Pona) -> Result<(), String>;
}

pub type PonaRegistryRef = Arc<dyn PonaRegistry + Send + Sync>;
