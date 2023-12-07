mod controllers;
mod domain;
mod infra;
mod services;
mod ui;

use std::sync::Arc;

use infra::persist::{PersistShuttleSessionRegistry, ShuttlePersistUserRegistry};
use rocket::fs::FileServer;
use services::{auth::AuthService, lili::LiliService, profile::ProfileService, user::UserService};
use shuttle_persist::PersistInstance;

use controllers::{hero::hero, login, login_post, profile_page, signup, signup_post, toki_lili_post};

use crate::infra::persist::{ShuttlePersistLiliRegistry, ShuttlePersistProfileRegistry};

#[macro_use]
extern crate rocket;

#[derive(Clone, Debug)]
struct AppState {
    pub persist: PersistInstance,
}

struct AppServices {
    pub lili_service: Arc<LiliService>,
    pub profile_service: Arc<ProfileService>,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_rocket::ShuttleRocket {
    dbg!(persist.clone().list().unwrap());
    persist.clear().unwrap();

    let services = AppServices {
        lili_service: Arc::new(LiliService::new(
            Arc::new(ShuttlePersistLiliRegistry {
                persist: persist.clone(),
            }),
            Arc::new(ShuttlePersistProfileRegistry {
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
    };

    let app = rocket::build()
        .mount(
            "/",
            routes![
                hero,
                profile_page,
                toki_lili_post,
                signup,
                signup_post,
                login,
                login_post
            ],
        )
        .mount("/static", FileServer::from("static"))
        .manage(services)
        .manage(AppState { persist });

    Ok(app.into())
}
