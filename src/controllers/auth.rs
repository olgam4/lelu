use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use rocket::{form::Form, State};

use crate::{
    controllers::hero,
    domain::{user::User, Profile, CurrentSession},
    infra::MaudTemplate,
    ui::{login_form, page, sign_up_form},
    AppServices,
};

#[post("/login", data = "<text>")]
pub async fn login_post(services: &State<AppServices>, text: Form<Signup>, current_session: CurrentSession) -> MaudTemplate {
    let session_id = services
        .auth_service
        .generate_session(&text.username, &text.password)
        .unwrap();

    // TODO: redirect to hero page
    hero(services, current_session).with_cookie(format!("session={}", session_id))
}

#[get("/logout")]
pub fn logout(services: &State<AppServices>, current_session: CurrentSession) -> MaudTemplate {
    services.auth_service.invalidate_session(&current_session.session_id).unwrap();

    hero(services, current_session).with_cookie("session=; Max-Age=0".to_string())

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
pub async fn signup_post(services: &State<AppServices>, text: Form<Signup>, current_session: CurrentSession) -> MaudTemplate {
    println!("signup post");

    let user = services.user_service.get_user(&text.username);
    if user.is_some() {
        return signup();
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(text.password.as_bytes(), &salt)
        .unwrap();

    let user = User {
        username: text.username.clone(),
        password_hash: password_hash.to_string(),
    };

    services.user_service.create_user(user).unwrap();

    let profile = Profile {
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
    };

    services.profile_service.create_profile(profile).unwrap();

    let session_id = services
        .auth_service
        .generate_session(&text.username, &text.password)
        .unwrap();

    // TODO: redirect to hero page
    hero(services, current_session).with_cookie(format!("session={}", session_id))
}
