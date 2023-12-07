use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use nanoid::nanoid;

use rocket::{form::Form, State};

use crate::{
    controllers::hello,
    domain::{user::User, LoggedInSession, Profile},
    infra::MaudTemplate,
    ui::{login_form, page, sign_up_form},
    AppState,
};

#[post("/login", data = "<text>")]
pub async fn login_post(state: &State<AppState>, text: Form<Signup>) -> MaudTemplate {
    println!("login post");
    let user = state
        .persist
        .load::<User>(format!("user_{}", text.username.clone()).as_str())
        .map_err(|_| login())
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

            hello::hello(state).with_cookie(format!("session={}", session.session_id))
        }
        Err(_) => login(),
    }
}

#[get("/signup")]
pub fn signup() -> MaudTemplate {
    page(sign_up_form(), "Signup")
}

#[get("/login")]
pub fn login() -> MaudTemplate {
    page(login_form(), "Login")
}

#[derive(FromForm)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

#[post("/signup", data = "<text>")]
pub async fn signup_post(state: &State<AppState>, text: Form<Signup>) -> MaudTemplate {
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

    login()
}
