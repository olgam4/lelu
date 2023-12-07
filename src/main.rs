mod components;
mod maud_custom;
mod page;
mod session;

use components::{login_form, sign_up_form, toast, FeedProps};
use itertools::Itertools;
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
    components::{lili, Profile},
    maud_custom::MaudTemplate,
    page::page,
    session::LoggedInSession,
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
async fn toki_lili_post(
    app_state: &State<AppState>,
    lili_form: Form<LiliForm>,
    session: LoggedInSession,
    ) -> MaudTemplate {
    let id = nanoid!();

    let new_lili = Lili {
        id: "1".to_string(),
        text: lili_form.text.clone(),
        username: session.username.clone(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    app_state
        .persist
        .save::<Lili>(format!("lili_{}", id.clone()).as_str(), new_lili.clone())
        .unwrap();

    let some_profile = app_state
        .persist
        .load::<Profile>(format!("profile_{}", session.username.clone()).as_str())
        .unwrap();

    MaudTemplate {
        string: lili(new_lili, some_profile),
        headers: Some(vec![(
            "HX-Trigger-After-Swap".to_string(),
            event("notify".to_string(), "toki+lili li sitelen".to_string()),
        )]),
    }
}

#[get("/")]
fn hello(state: &State<AppState>) -> MaudTemplate {
    let some_profile = state.persist.load::<Profile>("profile_gamachexx").unwrap();

    let lilis = state
        .persist
        .list()
        .unwrap()
        .iter()
        .filter(|key| key.starts_with("lili_"))
        .map(|key| state.persist.load::<Lili>(key).unwrap())
        .collect::<Vec<Lili>>();

    let lilis: Vec<Lili> = lilis
        .into_iter()
        .sorted_by(|a, b| b.timestamp.cmp(&a.timestamp))
        .collect();

    let feed_props = FeedProps {
        lilis: lilis
            .iter()
            .map(|c| (c.to_owned(), some_profile.clone()))
            .collect(),
    };

    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (toki_lili())
                    (feed(feed_props))
                }
                (trending())
            }
        },
        "Toki Lili",
    )
}

#[get("/profile/<username>")]
fn profile_page(
    state: &State<AppState>,
    username: String,
) -> MaudTemplate {
    let some_profile = state.persist.load::<Profile>(format!("profile_{}", username).as_str()).unwrap();

    let lilis = state
        .persist
        .list()
        .unwrap()
        .iter()
        .filter(|key| key.starts_with("lili_"))
        .map(|key| state.persist.load::<Lili>(key).unwrap())
        .sorted_by(|a, b| b.timestamp.cmp(&a.timestamp))
        .collect::<Vec<Lili>>();

    let feed_props = FeedProps {
        lilis: lilis
            .iter()
            .map(|c| (c.to_owned(), some_profile.clone()))
            .collect(),
    };

    page(
        html! {
            div class="main-page" {
                (nav())
                div {
                    (profile(some_profile.clone()))
                    (feed(feed_props))
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

    let user = state
        .persist
        .load::<User>(format!("user_{}", text.username.clone()).as_str())
        .map_err(|_| signup())
        .ok();
    if user.is_some() {
        return signup();
    }

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

    state
        .persist
        .save::<Profile>(
            format!("profile_{}", text.username.clone()).as_str(),
            Profile {
                username: text.username.clone(),
                name: text.username.clone(),
                avatar: format!(
                    "https://api.dicebear.com/7.x/big-smile/svg?seed={}",
                    text.username.clone()
                ),
                bio: "".to_string(),
                website: "".to_string(),
                location: "".to_string(),
                birthday: "".to_string(),
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
        None => return login(),
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    let result = Argon2::default().verify_password(text.password.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => {
            let session = LoggedInSession {
                session_id: nanoid!(32),
                username: text.username.clone(),
                expires: chrono::Utc::now().timestamp() + 60 * 60 * 8,
            };

            dbg!(session.session_id.clone());

            state
                .persist
                .save::<LoggedInSession>(
                    format!("session_{}", session.session_id.clone()).as_str(),
                    session.clone(),
                )
                .unwrap();

            dbg!(session.clone());

            hello(state).with_cookie(format!("session={}", session.session_id))
        }
        Err(_) => login(),
    }
}

#[get("/signup")]
fn signup() -> MaudTemplate {
    page(sign_up_form(), "Signup")
}

#[get("/login")]
fn login() -> MaudTemplate {
    page(login_form(), "Login")
}

#[get("/secure")]
fn secure(_session: LoggedInSession) -> MaudTemplate {
    page(html! { div { "Secure" } }, "Secure")
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
                secure,
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
