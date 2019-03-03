use crate::auth::IdP;
use crate::auth::IdentityProvider;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;
use rocket::State;

use rocket::http::{RawStr, Status};
use rocket::request::{Form, FromFormValue};

use crate::models::*;
use crate::schema;
use crate::Database;

#[derive(FromForm)]
pub struct CreateLink {
    /// This should be a path relative to the root, excluding leading slashes.
    origin: URLText,
    /// This must be a fully resolved link, including protocol.
    dest: String,
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

    for c in string.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }

    if string == "api" {
        return false;
    }

    return true;
}

#[post("/", data = "<link>")]
pub fn new_link(conn: Database, link: Form<CreateLink>, idp: State<IdP>) -> Status {
    use schema::links;

    idp.provider.can_create_mapping(String::from("asdf"));

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
}

#[delete("/", data = "<link>")]
pub fn delete_link(conn: Database, link: Form<DeleteLink>) -> Status {
    use schema::links::dsl::*;

    match diesel::delete(links.filter(origin.eq(&link.origin.0))).execute(&conn.0) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
