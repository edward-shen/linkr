#![feature(proc_macro_hygiene, decl_macro, bind_by_move_pattern_guards)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use std::env;

use rocket::http::{RawStr, Status};
use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket::Request;

use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

mod models;
mod schema;

use models::*;

#[database("linkrdb")]
struct Database(PgConnection);

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
    r#"Hello! Welcome to linkr, a easy-to-use URL shortener and/or URL prettifier.

To make a redirected URL via cURL...
    ... from "your-domain.com/hello"
    ... to "google.com"
    ... where your password is "potato"

    curl -XPOST -d "origin=hello&dest=https://google.com&password=potato" your-domain.com/api/link

The server will respond with one of the following:
    201 CREATED                 The link was successfully created.
    401 UNAUTHORIZED            The password provided was incorrect.
    409 CONFLICT                A link already exists on this domain.
    500 INTERNAL SERVER ERROR   Something bad happened and you should file a bug report.

To delete a URL via cURL...
    ... from "you-domain.com/hello"
    ... where your password is "potato"

    curl -XDELETE -d "origin=hello&password=potato" your-domain/api/link

The server will respond with one of the following:
    200 OK                      The link, if it exists, was deleted.
    401 UNAUTHORIZED            The password provided was incorrect.
    500 INTERNAL SERVER ERROR   Please file a bug report.
"#
}

#[derive(FromForm)]
struct CreateLink {
    origin: URLText,
    dest: String,
    password: String,
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

    if link.password != env::var("LINKR_PASSWORD").unwrap() {
        return Status::Unauthorized;
    }

    let new_link = NewLink {
        origin: link.origin.0.clone(),
        dest: link.dest.clone(),
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
    password: String,
}

#[delete("/api/link", data = "<link>")]
fn delete_link(conn: Database, link: Form<DeleteLink>) -> Status {
    use schema::links::dsl::*;

    if link.password != env::var("LINKR_PASSWORD").unwrap() {
        return Status::Unauthorized;
    }

    match diesel::delete(links.filter(origin.eq(&link.origin.0))).execute(&conn.0) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

fn main() {
    dotenv().ok();

    env::var("LINKR_PASSWORD")
        .expect("LINKR_PASSWORD env variable not found. Please put it in .env or declare it!");

    rocket::ignite()
        .mount("/", routes![introduction, index, new_link, delete_link])
        // .register(catchers![not_found])
        .attach(Database::fairing())
        .launch();
}
