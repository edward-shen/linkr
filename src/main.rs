#![feature(proc_macro_hygiene, decl_macro, bind_by_move_pattern_guards)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate chrono;

mod auth;
mod models;
mod schema;

use std::env;

use rocket::http::{RawStr, Status};
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;

use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

// use chrono::naive::NaiveDateTime;

use models::*;

#[get("/<url>")]
fn index(conn: Database, url: String) -> Option<Redirect> {
    use schema::links::dsl::*;
    let results = links
        .filter(origin.eq(url))
        .load::<Link>(&conn.0)
        .expect("Failed to get link table");
    match results.len() {
        0 => None,
        1 => Some(Redirect::temporary(results[0].dest.clone())),
        _ => panic!(
            "Multiple results found for source path {}. Postgres constraints violated!",
            results.len()
        ),
    }
}

#[get("/")]
fn introduction() -> &'static str {
    "For help, please view https://github.com/edward-shen/linkr"
}

#[derive(FromForm)]
struct CreateLink {
    origin: URLText,
    dest: String,
}

struct URLText(String);

impl<'v> FromFormValue<'v> for URLText {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<URLText, &'v RawStr> {
        match form_value.parse::<String>() {
            Ok(link) if is_valid_origin(&link) => Ok(URLText(link)),
            _ => Err(form_value),
        }
    }
}

fn is_valid_origin(string: &String) -> bool {
    if string.is_empty() {
        return false;
    };

    for c in string.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }

    return true;
}
#[post("/api/link", data = "<link>")]
fn new_link(conn: Database, link: Form<CreateLink>) -> Status {
    use schema::links;

    let new_link = NewLink {
        origin: link.origin.0.clone(),
        dest: link.dest.clone(),
        owner: None,
        expire_date: None,
        expire_clicks: None,
    };

    match diesel::insert_into(links::table)
        .values(&new_link)
        .get_result::<Link>(&conn.0)
    {
        Ok(_) => Status::Created,
        Err(DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            Status::Conflict
        }
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
struct DeleteLink {
    origin: URLText,
}

#[delete("/api/link", data = "<link>")]
fn delete_link(conn: Database, link: Form<DeleteLink>) -> Status {
    use schema::links::dsl::*;

    match diesel::delete(links.filter(origin.eq(&link.origin.0))).execute(&conn.0) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[database("linkrdb")]
struct Database(PgConnection);

fn main() {
    dotenv().ok();

    run_migrations();

    rocket::ignite()
        .mount("/", routes![introduction, index, new_link, delete_link])
        // .register(catchers![not_found])
        .attach(Database::fairing())
        .launch();
}

fn run_migrations() {
    embed_migrations!();

    let database_url = parse_database_env();
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Could not connect to {}", database_url));

    embedded_migrations::run(&connection).expect("Could not run migrations!");
}

fn parse_database_env() -> String {
    let database_url = env::var("ROCKET_DATABASES").expect("ROCKET_DATABASES must be set!");
    let database_url = database_url.as_str();

    // This is really gross but I don't know of a better way
    // FIXME: make this less gross
    // Value of database_url is {linkrdb={url=postgres://linkr@localhost/linkrdb}}
    // but I have no idea what langauge it's in?
    String::from(
        &database_url[database_url.rfind("=").unwrap() + 1..database_url.rfind("}").unwrap() - 1],
    )
}
