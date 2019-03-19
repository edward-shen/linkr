use rocket::request::{Form, FromFormValue};

use std::env;

#[derive(FromForm)]
pub struct Login {
    username: Option<String>,
    password: String,
}

#[post("/login", data = "<login>")]
pub fn login<'a>(login: Form<Login>) -> Option<&'a str> {
    let database_url = env::var("ENABLE_LOGIN").unwrap_or_default();

    // Recast String to &str, looks cleaner.
    match &*database_url {
        "true" => Some("henlo"),
        _ => None,
    }
}

#[post("/create", data = "<login>")]
pub fn create_user(login: Form<Login>) {}
