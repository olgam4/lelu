use rocket::{form::Form, State};

use crate::{
    domain::LoggedInSession, infra::MaudTemplate, services::lili::NewLili, ui, AppServices,
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
    lili_form: Form<LiliForm>,
    session: LoggedInSession,
    services: &State<AppServices>,
) -> MaudTemplate {
    let new_lili = NewLili {
        text: lili_form.text.clone(),
        from_username: session.username.clone(),
    };

    let (lili, profile) = services.lili_service.create_lili(new_lili).expect("should be logged in");

    MaudTemplate {
        string: ui::lili(lili, profile),
        headers: Some(vec![(
            "HX-Trigger-After-Swap".to_string(),
            event("notify".to_string(), "toki+lili li sitelen".to_string()),
        )]),
    }
}
