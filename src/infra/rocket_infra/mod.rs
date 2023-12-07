use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request, State,
};

use crate::{
    domain::{CurrentSession, LoggedInSession},
    AppState,
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedInSession {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let state = request
            .guard::<&State<AppState>>()
            .await
            .expect("should be instantiaed");
        let cookies = request.cookies();

        dbg!(state.persist.clone().list().unwrap());

        let session = state
            .persist
            .list()
            .unwrap()
            .iter()
            .filter(|key| key.starts_with("session"))
            .map(|key| state.persist.load::<LoggedInSession>(&key).unwrap())
            .collect::<Vec<LoggedInSession>>()
            .into_iter()
            .filter(|session| {
                cookies
                    .get("session")
                    .map(|cookie| cookie.value() == session.session_id)
                    .unwrap_or(false)
            })
            .next();

        match session {
            Some(session) => request::Outcome::Success(session),
            None => request::Outcome::Error((Status::Unauthorized, "Unauthorized".to_string())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentSession {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let state = request
            .guard::<&State<AppState>>()
            .await
            .expect("should be instantiaed");
        let cookies = request.cookies();

        let session = state
            .persist
            .list()
            .unwrap()
            .iter()
            .filter(|key| key.starts_with("session"))
            .map(|key| state.persist.load::<CurrentSession>(&key))
            .filter_map(Result::ok)
            .collect::<Vec<CurrentSession>>()
            .into_iter()
            .filter(|session| {
                cookies
                    .get("session")
                    .map(|cookie| cookie.value() == session.session_id)
                    .unwrap_or(false)
            })
            .next();

        match session {
            Some(session) => request::Outcome::Success(session),
            None => request::Outcome::Success(CurrentSession {
                session_id: "".to_string(),
                username: None,
            }),
        }
    }
}
