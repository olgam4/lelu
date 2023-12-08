use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lili {
    pub id: String,
    pub text: String,
    pub username: String,
    pub timestamp: i64,
    pub liked_by_me: bool,
    pub ponas: i64,
}

pub trait LiliRegistry: Send + Sync {
    fn get_all(&self) -> Vec<Lili>;
    fn create_lili(&self, lili: Lili) -> Result<(), String>;
}

pub type LiliRegistryRef = Arc<dyn LiliRegistry>;
