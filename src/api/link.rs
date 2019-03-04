use std::num::ParseIntError;
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

use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(FromForm)]
pub struct CreateLink {
    /// This should be a path relative to the root, excluding leading slashes.
    origin: URLText,
    /// This must be a fully resolved link, including protocol.
    dest: String,
    ts: u64,
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
    if string.is_empty() {
        return false;
    };

    if string == "api" {
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
        if link.hash.is_none() {
            println!("No hash found");
            return Status::NotAcceptable;
        }

        if !verify_hash(
            key,
            format!("origin={}&dest={}&ts={}", link.origin.0, link.dest, link.ts),
            link.hash.clone().unwrap(),
        ) {}

        // Check timestamp
        let cur_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("Could not get system time!"),
        };

        // If cur_time - link.ts < 0, the value will underflow; therefore no need to check.
        if cur_time - link.ts > 5 {
            println!("Time too different");
            return Status::NotAcceptable;
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

fn verify_hash(key: String, value: String, hash: String) -> bool {
    let mut mac = Hmac::<Sha256>::new_varkey(key.as_bytes()).unwrap();
    mac.input(value.as_bytes());

    let hash: Vec<u8> = decode_hex(&hash).unwrap();

    mac.verify(&hash).is_ok()
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[derive(FromForm)]
pub struct DeleteLink {
    origin: URLText,
    token: Option<String>,
}

#[delete("/", data = "<link>")]
pub fn delete_link(conn: Database, link: Form<DeleteLink>, idp: State<&IdP>) -> Status {
    use schema::links::dsl::*;
    if !idp
        .provider
        .can_delete_own_mapping(link.token.clone().unwrap_or_default())
    {
        return Status::Unauthorized;
    }

    match diesel::delete(links.filter(origin.eq(&link.origin.0))).execute(&conn.0) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
