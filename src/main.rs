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

mod api;
mod auth;
mod models;
mod schema;

use std::env;

use rocket::response::Redirect;

use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;

// use chrono::naive::NaiveDateTime;

use api::admin::*;
use api::link::*;
use api::user::*;

use models::*;

#[get("/<url>")]
fn url_resolver(conn: Database, url: String) -> Option<Redirect> {
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
fn index() -> &'static str {
    "For help, please view https://github.com/edward-shen/linkr"
}

#[database("linkrdb")]
pub struct Database(PgConnection);

fn main() {
    dotenv().ok();

    run_migrations();

    rocket::ignite()
        .mount("/", routes![index, url_resolver])
        .mount("/api/link/", routes![new_link, delete_link])
        .mount("/api/user/", routes![login, create_user])
        .mount("/api/admin/", routes![view_stats])
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
