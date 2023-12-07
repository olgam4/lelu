use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::domain::{LoggedInSession, SessionRegistryRef, UserRegistryRef};

pub struct AuthService {
    pub user_registry: UserRegistryRef,
    pub session_registry: SessionRegistryRef,
}

impl AuthService {
    pub fn new(user_registry: UserRegistryRef, session_registry: SessionRegistryRef) -> Self {
        Self {
            user_registry,
            session_registry,
        }
    }

    pub fn generate_session(&self, username: &str, password: &str) -> Result<String, String> {
        let user = self
            .user_registry
            .get_user(username)
            .ok_or("User not found".to_string())?;
        let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
        let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

        if result.is_err() {
            return Err("Invalid password".to_string());
        }

        let session = LoggedInSession {
            session_id: nanoid::nanoid!(32),
            username: username.to_string(),
            expires: chrono::Utc::now().timestamp() + 60 * 60 * 8,
        };

        self.session_registry.create_session(session.clone())?;

        Ok(session.session_id)
    }
}
