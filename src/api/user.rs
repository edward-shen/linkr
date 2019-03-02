use rocket::request::{Form, FromFormValue};

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
pub fn login(login: Form<Login>) {}

#[post("/create", data = "<login>")]
pub fn create_user(login: Form<Login>) {}
