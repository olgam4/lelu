use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use rocket::{
    form::Form,
    response::{Redirect, Responder},
    State,
};

use crate::{
    domain::{user::User, CurrentSession, Profile},
    infra::MaudTemplate,
    ui::{login_form, page, sign_up_form},
    AppServices,
};

pub struct RedirectWithCookie {
    redirect: Redirect,
    cookie: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for RedirectWithCookie {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let mut response = self.redirect.respond_to(request)?;
        let base_cookie = "; HttpOnly; SameSite=Strict; Path=/; Max-Age=31536000; Secure; ";
        let cookie = format!("{}{}", self.cookie, base_cookie);
        response.set_raw_header("Set-Cookie", cookie);
        Ok(response)
    }
}

#[post("/login", data = "<text>")]
pub async fn login_post(services: &State<AppServices>, text: Form<Signup>) -> RedirectWithCookie {
    let session_id = services
        .auth_service
        .generate_session(&text.username, &text.password);

    if session_id.is_err() {
        return RedirectWithCookie {
            redirect: Redirect::to("/login"),
            cookie: "session=; Max-Age=0".to_string(),
        };
    }
    let session_id = session_id.expect("should be instantiaed");

    RedirectWithCookie {
        redirect: Redirect::to("/"),
        cookie: format!("session={}", session_id),
    }
}

#[get("/logout")]
pub fn logout(
    services: &State<AppServices>,
    current_session: CurrentSession,
) -> RedirectWithCookie {
    let _ = services
        .auth_service
        .invalidate_session(&current_session.session_id);

    RedirectWithCookie {
        redirect: Redirect::to("/"),
        cookie: "session=; Max-Age=0".to_string(),
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
pub async fn signup_post(services: &State<AppServices>, text: Form<Signup>) -> RedirectWithCookie {
    let err_redirect = RedirectWithCookie {
        redirect: Redirect::to("/signup"),
        cookie: "session=; Max-Age=0".to_string(),
    };

    let user = services.user_service.get_user(&text.username);
    if user.is_some() {
        return err_redirect;
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(text.password.as_bytes(), &salt)
        .expect("argon2 hashing should work");

    let user = User {
        username: text.username.clone(),
        password_hash: password_hash.to_string(),
    };

    let result = services.user_service.create_user(user);
    if result.is_err() {
        return err_redirect;
    }

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

    let result = services.profile_service.create_profile(profile);
    if result.is_err() {
        return err_redirect;
    }

    let session_id = services
        .auth_service
        .generate_session(&text.username, &text.password);

    if session_id.is_err() {
        return err_redirect;
    }
    let session_id = session_id.expect("should be instantiaed");

    RedirectWithCookie {
        redirect: Redirect::to("/"),
        cookie: format!("session={}", session_id),
    }
}
