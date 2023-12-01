mod components;
mod maud_custom;
mod page;

use components::{login_form, sign_up_form, toast};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;

use maud::html;
use rocket::{form::Form, fs::FileServer, State};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

use crate::{
    components::{feed, nav, profile, toki_lili, trending},
    maud_custom::MaudTemplate,
};
use crate::{
    components::{lili, Profile},
    page::page,
};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lili {
    pub id: String,
    pub text: String,
    pub username: String,
    pub timestamp: i64,
}

#[derive(FromForm)]
struct LiliForm {
    pub text: String,
}

pub fn event(name: String, info: String) -> String {
    format!("{{ \"{}\": \"{}\" }}", name, info)
}

#[post("/toki_lili", data = "<lili_form>")]
async fn toki_lili_post(app_state: &State<AppState>, lili_form: Form<LiliForm>) -> MaudTemplate {
    let id = nanoid!();

    let new_lili = Lili {
        id: "1".to_string(),
        text: lili_form.text.clone(),
        username: "jan+telo".to_string(),
        timestamp: 1701008545,
    };

    app_state
        .persist
        .save::<Lili>(format!("lili_{}", id.clone()).as_str(), new_lili.clone())
        .unwrap();

    let some_profile = Profile {
        username: "jan+sona".to_string(),
        name: "jan sona".to_string(),
        avatar: format!(
            "https://api.dicebear.com/7.x/big-smile/svg?seed={}",
            "jan sona"
        ),
        bio: "mi wile moku e kili e telo e pan".to_string(),
        website: "https://jan.sona".to_string(),
        location: "ma+ali".to_string(),
        birthday: "2000-01-01".to_string(),
    };

    MaudTemplate {
        string: lili(new_lili, some_profile),
        headers: Some(vec![(
            "HX-Trigger-After-Swap".to_string(),
            event("notify".to_string(), "toki+lili li sitelen".to_string()),
        )]),
    }
}

#[get("/")]
fn hello() -> MaudTemplate {
    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (toki_lili())
                    (feed())
                }
                (trending())
            }
        },
        "Toki Lili",
    )
}

#[get("/profile")]
fn profile_page() -> MaudTemplate {
    let some_profile = Profile {
        username: "jan+sona".to_string(),
        name: "jan sona".to_string(),
        avatar: format!(
            "https://api.dicebear.com/7.x/big-smile/svg?seed={}",
            "jan sona"
        ),
        bio: "mi wile moku e kili e telo e pan".to_string(),
        website: "https://jan.sona".to_string(),
        location: "ma+ali".to_string(),
        birthday: "2000-01-01".to_string(),
    };
    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (profile(some_profile.clone()))
                }
                div {}
            }
        },
        "Profile",
    )
}

#[derive(FromForm)]
struct Signup {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    pub username: String,
    pub password_hash: String,
}

#[post("/signup", data = "<text>")]
async fn signup_post(state: &State<AppState>, text: Form<Signup>) -> MaudTemplate {
    println!("signup post");

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(text.password.as_bytes(), &salt)
        .unwrap();

    state
        .persist
        .save::<User>(
            format!("user_{}", text.username.clone()).as_str(),
            User {
                username: text.username.clone(),
                password_hash: password_hash.to_string(),
            },
        )
        .unwrap();

    toast(password_hash.to_string()).into()
}

#[post("/login", data = "<text>")]
async fn login_post(state: &State<AppState>, text: Form<Signup>) -> MaudTemplate {
    println!("login post");
    let user = state
        .persist
        .load::<User>(format!("user_{}", text.username.clone()).as_str())
        .map_err(|_| toast("User not found".to_string()))
        .ok();

    let user = match user {
        Some(user) => user,
        None => return toast("User not found".to_string()).into(),
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    let result = Argon2::default().verify_password(text.password.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => toast("Logged in".to_string()),
        Err(_) => toast("User not found".to_string()),
    }
    .into()
}

#[get("/signup")]
fn signup() -> MaudTemplate {
    page(sign_up_form(), "Signup")
}

#[get("/login")]
fn login() -> MaudTemplate {
    page(login_form(), "Login")
}

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
