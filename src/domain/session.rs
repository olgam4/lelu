use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedInSession {
    pub session_id: String,
    pub username: String,
    pub expires: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentSession {
    pub session_id: String,
    pub username: Option<String>,
}

pub trait SessionRegistry: Send + Sync {
    fn get_session(&self, session_id: &str) -> Option<LoggedInSession>;
    fn create_session(&self, session: LoggedInSession) -> Result<(), String>;
    fn delete_session(&self, session_id: &str) -> Result<(), String>;
}

pub type SessionRegistryRef = Arc<dyn SessionRegistry>;
