use diesel::prelude::*;
use diesel::result::Error::DatabaseError;

use rocket::http::{RawStr, Status};
use rocket::request::{Form, FromFormValue};

use crate::models::*;
use crate::schema;
use crate::Database;

#[derive(FromForm)]
pub struct CreateLink {
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

#[post("/", data = "<link>")]
pub fn new_link(conn: Database, link: Form<CreateLink>) -> Status {
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
