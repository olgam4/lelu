mod infra;
mod domain;
mod ui;
mod controllers;

use rocket::fs::FileServer;
use shuttle_persist::PersistInstance;

use controllers::{hello::hello, login, login_post, signup, signup_post, toki_lili_post, profile_page};

#[macro_use]
extern crate rocket;


#[derive(FromForm)]
struct LiliForm {
    pub text: String,
}



#[derive(FromForm)]
struct Signup {
    pub username: String,
    pub password: String,
}



#[derive(Clone, Debug)]
struct AppState {
    pub persist: PersistInstance,
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_rocket::ShuttleRocket {
    dbg!(persist.clone().list().unwrap());

    let app = rocket::build()
        .mount(
            "/",
            routes![
                hello,
                profile_page,
                toki_lili_post,
                signup,
                signup_post,
                login,
                login_post
            ],
        )
        .mount("/static", FileServer::from("static"))
        .manage(AppState { persist });

    Ok(app.into())
}
