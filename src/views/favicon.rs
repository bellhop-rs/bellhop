use rocket::response::Redirect;

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::permanent("/static/img/favicon.ico")
}
