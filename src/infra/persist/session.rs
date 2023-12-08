use shuttle_persist::PersistInstance;

use crate::domain::SessionRegistry;

pub struct PersistShuttleSessionRegistry {
    pub persist: PersistInstance,
}

const SESSION_PREFIX: &str = "session_";

impl SessionRegistry for PersistShuttleSessionRegistry {
    fn get_session(&self, session_id: &str) -> Option<crate::domain::LoggedInSession> {
        self.persist
            .load::<crate::domain::LoggedInSession>(
                format!("{}{}", SESSION_PREFIX, session_id).as_str(),
            )
            .ok()
    }

    fn create_session(&self, session: crate::domain::LoggedInSession) -> Result<(), String> {
        self.persist
            .save(
                format!("{}{}", SESSION_PREFIX, session.session_id).as_str(),
                &session,
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn delete_session(&self, session_id: &str) -> Result<(), String> {
        let _ = self
            .persist
            .remove(format!("{}{}", SESSION_PREFIX, session_id).as_str())
            .map_err(|e| e.to_string());

        Ok(())
    }
}
