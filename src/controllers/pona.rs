use rocket::State;

use crate::{domain::LoggedInSession, infra::MaudTemplate, ui, AppServices};

#[post("/pona/<id>")]
pub async fn pona_lili_post(
    session: LoggedInSession,
    services: &State<AppServices>,
    id: String,
) -> MaudTemplate {
    println!("hiii");
    let username = session.username;
    let _ = services.lili_service.pona_lili(&id, &username);

    ui::pona_active(id, 1).into()
}

#[post("/ike/<id>")]
pub async fn ike_lili_post(
    session: LoggedInSession,
    services: &State<AppServices>,
    id: String,
) -> MaudTemplate {
    println!("hiii");
    let username = session.username;
    let _ = services.lili_service.ike_lili(&id, &username);

    ui::pona_inactive(id, 0).into()
}
