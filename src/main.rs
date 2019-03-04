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
extern crate hmac;
extern crate sha2;

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

use auth::AuthMethod;

use models::*;

#[database("linkrdb")]
pub struct Database(PgConnection);

static mut IDP_PROVIDER: Option<auth::IdP> = None;
static mut AUTH_METHOD: Option<AuthMethod> = None;

macro_rules! start {
    () => {
        /**
         * A macro is needed because of the interactions between lazy_static and
         * the rust compiler.
         *
         * It's better to explain the reasoning as a story if anything:
         *
         * PROVIDER isn't defined when no feature is enabled, so it fails to
         * compile. We could define a dummy PROVIDER variable, but then
         * lazy_static complains because there's a duplicate definition of
         * PROVIDER, since it does some shenanigans and need unique variable names.
         *
         * Thus, the next reasonable step is to try and turn this into a
         * function. The issue then arises with how to type the provider as a
         * parameter. Simply put, you can't, because lazy_static dynamically
         * generates a type, which isn't visible to the RLS and fails to compile.
         *
         * So what do you do? Write a macro that effectively acts as a function
         * to deduplicate code.
         */
        use api::admin::*;
        use api::link::*;
        use api::user::*;

        // Safe, we are single-threaded at this point.
        unsafe {
            IDP_PROVIDER = Some(auth::IdP {
                provider: &*PROVIDER,
                auth_method: &AUTH_METHOD.as_ref().unwrap(),
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

    unsafe {
        AUTH_METHOD = Some(AuthMethod::get_type(
            &env::var("AUTH_METHOD").expect("AUTH_METHOD must be set!"),
        ));

        match AUTH_METHOD {
            Some(AuthMethod::NoAuth) => {
                lazy_static! {
                    static ref PROVIDER: auth::no_auth::Provider = auth::no_auth::Provider {};
                }
                start!();
            }

            Some(AuthMethod::PSK) => {
                lazy_static! {
                    static ref TOKEN: String = env::var("PRESHARED_TOKEN").unwrap();
                    static ref PROVIDER: auth::preshared_key::Provider =
                        auth::preshared_key::Provider { key: TOKEN.clone() };
                }
                start!();
            }

            None => panic!("Auth method not set"),
        }
    }
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
