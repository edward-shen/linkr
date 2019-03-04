#![feature(proc_macro_hygiene, decl_macro, bind_by_move_pattern_guards)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

mod api;
mod auth;
mod models;
mod schema;

use std::env;
use std::error::Error;

use rocket::response::Redirect;

use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;

// use chrono::naive::NaiveDateTime;

use models::*;

#[database("linkrdb")]
pub struct Database(PgConnection);

static mut IDP_PROVIDER: Option<auth::IdP> = None;

#[allow(unused)] // This is used, just only in conditional branches so RLS doesn't see it
macro_rules! start {
    () => {
        use api::admin::*;
        use api::link::*;
        use api::user::*;

        unsafe {
            IDP_PROVIDER = Some(auth::IdP {
                provider: &*PROVIDER,
            });

            rocket::ignite()
                .mount("/", routes![index, url_resolver])
                .mount("/api/link/", routes![new_link, delete_link])
                .mount("/api/user/", routes![login, create_user])
                .mount("/api/admin/", routes![view_stats])
                // .register(catchers![not_found])
                .manage(IDP_PROVIDER.as_ref().unwrap())
                .attach(Database::fairing())
                .launch();
        }
    };
}

fn main() {
    dotenv().ok();

    run_migrations();

    #[cfg(feature = "no_auth")]
    {
        lazy_static! {
            static ref PROVIDER: auth::no_auth::Provider = auth::no_auth::Provider {};
        }
        start!();
    }

    #[cfg(feature = "single_user")]
    {
        lazy_static! {
            static ref env_var: String = env::var("PRESHARED_TOKEN").unwrap();
            static ref PROVIDER: auth::single_user::Provider = auth::single_user::Provider {
                key: env_var.clone()
            };
        }
        start!();
    }

    // TODO: Analyze safety
}

/// Automated DB migration, to allow an easy-to-install binary.
fn run_migrations() {
    embed_migrations!();

    embedded_migrations::run(&get_db_connection()).expect("Could not run migrations!");
}

pub fn get_db_connection() -> diesel::PgConnection {
    let database_url = parse_database_env();
    match PgConnection::establish(&database_url) {
        Ok(conn) => conn,
        Err(e) => panic!(format!("\n{}", e.description())),
    }
}

fn parse_database_env() -> String {
    let database_url = env::var("ROCKET_DATABASES").expect("ROCKET_DATABASES must be set!");
    let database_url = database_url.as_str();

    // This is really gross but I don't know of a better way
    // FIXME: make this less gross
    // Value of database_url is {linkrdb={url=postgres://linkr@localhost/linkrdb}}
    // but I have no idea what language it's in?
    String::from(
        &database_url[database_url.rfind("=").unwrap() + 1..database_url.rfind("}").unwrap() - 1],
    )
}

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
    "For help, please view https://github.com/edward-shen/linkr\n"
}

#[cfg(test)]
mod init_test {
    mod parse_env {
        use super::super::*;

        #[test]
        fn standard_input() {
            env::set_var(
                "ROCKET_DATABASES",
                "{linkrdb={url=postgres://linkr@localhost/linkrdb}}",
            );
            assert_eq!("postgres://linkr@localhost/linkrdb", parse_database_env());
        }

        #[test]
        #[should_panic]
        fn empty_input() {
            env::remove_var("ROCKET_DATABASES");
            parse_database_env();
        }

        #[test]
        fn invalid_input_but_parses() {
            env::set_var(
                "ROCKET_DATABASES",
                "{linkrdb====postgres://linkr@localhost/linkrdb}24gafsdgdsdgafd",
            );
            assert_eq!("postgres://linkr@localhost/linkrd", parse_database_env());
        }

        #[test]
        #[should_panic]
        fn invalid_input() {
            env::set_var("ROCKET_DATABASES", "postgres://linkr@localhost/linkrdb");
            parse_database_env();
        }
    }
}
