use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lili {
    pub id: String,
    pub text: String,
    pub username: String,
    pub timestamp: i64,
}
