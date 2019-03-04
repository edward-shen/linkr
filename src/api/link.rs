use std::time::SystemTime;

use rocket::http::{RawStr, Status};
use rocket::request::{Form, FromFormValue};
use rocket::State;

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use crate::auth::IdP;
use crate::models::*;
use crate::schema;
use crate::Database;

use super::verify_hash;

#[derive(FromForm)]
pub struct CreateLink {
    /// This should be a path relative to the root, excluding leading slashes.
    origin: URLText,
    /// This must be a fully resolved link, including protocol.
    dest: String,
    ts: Option<u64>,
    key: Option<String>,
    hash: Option<String>,
}

struct URLText(String);

/// Custom string validation for checking if the origin URL is valid.
impl<'v> FromFormValue<'v> for URLText {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<URLText, &'v RawStr> {
        match form_value.parse::<String>() {
            Ok(link) if is_valid_origin(&link) => Ok(URLText(link)),
            _ => Err(form_value),
        }
    }
}

/// Checks whether or not the user-provided string is a valid link. There are
/// a few limits to a valid link:
/// - The link must not be empty
/// - The link can only consist of alphanumeric characters, hyphens, or dashes.
/// - The following routes are forbidden:
///     - /api
fn is_valid_origin(string: &String) -> bool {
    if string.is_empty() || string == "api" {
        return false;
    }

    for c in string.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }

    return true;
}

#[post("/", data = "<link>")]
pub fn new_link(conn: Database, link: Form<CreateLink>, idp: State<&IdP>) -> Status {
    use schema::links;

    if !idp.provider.get_key().is_some()
        && !idp
            .provider
            .can_create_mapping(link.key.clone().unwrap_or_default())
    {
        return Status::Unauthorized;
    }

    // Require key if IdP has symmetric key
    if let Some(key) = idp.provider.get_key() {
        let ts = link.ts.unwrap_or_default();
        let result = handle_psk(
            key,
            format!("origin={}&dest={}&ts={}", link.origin.0, link.dest, ts),
            link.hash.clone(),
            ts,
        );

        match result {
            Some(error) => return error,
            None => (),
        }
    }

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
        Err(DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct DeleteLink {
    origin: URLText,
    key: Option<String>,
    hash: Option<String>,
    ts: Option<u64>,
}

#[delete("/", data = "<link>")]
pub fn delete_link(conn: Database, link: Form<DeleteLink>, idp: State<&IdP>) -> Status {
    use schema::links::dsl::*;
    if !idp.provider.get_key().is_some()
        && !idp
            .provider
            .can_delete_own_mapping(link.key.clone().unwrap_or_default())
    {
        return Status::Unauthorized;
    }

    // Require key if IdP has symmetric key
    if let Some(key) = idp.provider.get_key() {
        let ts = link.ts.unwrap_or_default();
        let result = handle_psk(
            key,
            format!("origin={}&ts={}", link.origin.0, ts),
            link.hash.clone(),
            ts,
        );

        match result {
            Some(error) => return error,
            None => (),
        }
    }

    match diesel::delete(links.filter(origin.eq(&link.origin.0))).execute(&conn.0) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

fn handle_psk(key: String, value: String, hash: Option<String>, ts: u64) -> Option<Status> {
    if hash.is_none() {
        return Some(Status::Unauthorized);
    }

    if !verify_hash(key, value, hash.clone().unwrap()) {
        return Some(Status::BadRequest);
    }

    // Check timestamp
    let cur_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("Could not get system time!"),
    };

    // If cur_time - link.ts < 0, the value will underflow; therefore no need to check.
    if cur_time - ts > 5 {
        return Some(Status::new(425, "Too Early"));
    }

    None
}
