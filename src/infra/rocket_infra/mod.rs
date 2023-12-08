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

        let session = state
            .persist
            .list()
            .expect("should be instantiaed")
            .iter()
            .filter(|key| key.starts_with("session"))
            .map(|key| state.persist.load::<LoggedInSession>(&key))
            .flatten()
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
            .map(|key| state.persist.load::<LoggedInSession>(&key))
            .flatten()
            .map(|session| CurrentSession {
                session_id: session.session_id,
                username: Some(session.username),
            })
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
