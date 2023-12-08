use std::sync::Arc;

use shuttle_persist::PersistInstance;

use crate::{
    infra::persist::{
        PersistShuttleSessionRegistry, ShuttlePersistLiliRegistry, ShuttlePersistPonaRegistry,
        ShuttlePersistProfileRegistry, ShuttlePersistUserRegistry,
    },
    services::{auth::AuthService, lili::LiliService, profile::ProfileService, user::UserService},
    AppServices,
};

pub fn generate_context(persist: PersistInstance) -> AppServices {
    AppServices {
        lili_service: Arc::new(LiliService::new(
            Arc::new(ShuttlePersistLiliRegistry {
                persist: persist.clone(),
            }),
            Arc::new(ShuttlePersistProfileRegistry {
                persist: persist.clone(),
            }),
            Arc::new(ShuttlePersistPonaRegistry {
                persist: persist.clone(),
            }),
        )),
        profile_service: Arc::new(ProfileService::new(Arc::new(
            ShuttlePersistProfileRegistry {
                persist: persist.clone(),
            },
        ))),
        auth_service: Arc::new(AuthService::new(
            Arc::new(ShuttlePersistUserRegistry {
                persist: persist.clone(),
            }),
            Arc::new(PersistShuttleSessionRegistry {
                persist: persist.clone(),
            }),
        )),
        user_service: Arc::new(UserService::new(Arc::new(ShuttlePersistUserRegistry {
            persist: persist.clone(),
        }))),
    }
}
