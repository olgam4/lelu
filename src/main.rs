mod context;
mod controllers;
mod domain;
mod infra;
mod services;
mod ui;

use std::sync::Arc;

use rocket::fs::FileServer;
use services::{auth::AuthService, lili::LiliService, profile::ProfileService, user::UserService};
use shuttle_persist::PersistInstance;

use controllers::{
    hero::hero, login, login_post, logout, pona_lili_post, profile_page, signup, signup_post,
    specific_profile_page, toki_lili_post, ike_lili_post,
};

use crate::context::generate_context;

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

    let services = generate_context(persist.clone());

    let app = rocket::build()
        .mount(
            "/",
            routes![
                hero,
                profile_page,
                specific_profile_page,
                toki_lili_post,
                pona_lili_post,
                ike_lili_post,
                signup,
                signup_post,
                login,
                login_post,
                logout,
            ],
        )
        .mount("/static", FileServer::from("static"))
        .manage(services)
        .manage(AppState { persist });

    Ok(app.into())
}
