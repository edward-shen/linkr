#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use std::collections::HashMap;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;


use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

use models::*;

#[database("linkrdb")]
struct Database(PgConnection);

#[get("/<url>")]
fn index(conn: Database, url: String) -> Option<Redirect> {
    use schema::links::dsl::*;
    let results = links.filter(origin.eq(url)).load::<Link>(&conn.0).expect("Failed to get link table");
    match results.len() {
        0 => None,
        1 => Some(Redirect::temporary(results[0].dest.clone())),
        _ => panic!("multiple destination entries!")
    }

    // use schema::users::dsl::*;
    // let results = users.load::<User>(&conn.0).expect("Failed");
    // for user in results {
    //     println!("username={}, pass={}, email={}", user.username, user.password, user.email);
    // }
    // format!("hello world, you tried to access {}", url)
}

#[get("/")]
fn login() -> Template {
    let mut context = HashMap::new();
    context.insert("potato", "field");
    Template::render("index", context)
}

#[get("/web/new_user")]
fn create_user(conn: Database) -> String {
    use schema::users;
    let new_user = NewUser {
        username: String::from("henlo"),
        password: String::from("world"),
        email: String::from("Henlo@world")
    };
    diesel::insert_into(users::table)
    .values(&new_user)
    .get_result::<User>(&conn.0)
    .expect("Could not add user");

    format!("ok!")
}

#[derive(FromForm)]
struct CreateLink {
    origin: String,
    dest: String,
    private: bool,
}

#[post("/api/new_link", data = "<link>")]
fn new_link(conn: Database, link: Form<CreateLink>) -> String {
    use schema::links;

    let new_link = NewLink {
        owner: None,
        origin: link.origin.clone(),
        dest: link.dest.clone(),
        is_private: link.private,
    };

    diesel::insert_into(links::table)
    .values(&new_link)
    .get_result::<Link>(&conn.0)
    .expect("Could not add link");

    format!("ok")
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, login, create_user, new_link])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .attach(Database::fairing())
        .launch();
}
