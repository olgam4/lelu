use nanoid::nanoid;
use rocket::{form::Form, State};

use crate::{
    domain::{Lili, LoggedInSession, Profile},
    infra::MaudTemplate,
    ui::lili,
    AppState,
};

pub fn event(name: String, info: String) -> String {
    format!("{{ \"{}\": \"{}\" }}", name, info)
}

#[derive(FromForm)]
pub struct LiliForm {
    pub text: String,
}

#[post("/toki_lili", data = "<lili_form>")]
pub async fn toki_lili_post(
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
