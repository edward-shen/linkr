use crate::models::Link;
use crate::schema;
use crate::Database;

use diesel::prelude::*;

use rocket_contrib::json::Json;

#[get("/stats?<src>")]
pub fn view_stats(conn: Database, src: String) -> Json<Vec<Link>> {
    use schema::links::dsl::*;
    let results = links
        .filter(origin.eq(src))
        .load::<Link>(&conn.0)
        .expect("Failed to get link table");

    Json(results)
}

#[get("/stats")]
pub fn view_stats_all(conn: Database) -> Json<Vec<Link>> {
    use schema::links::dsl::*;
    let results = links
        .load::<Link>(&conn.0)
        .expect("Failed to get link table");

    Json(results)
}
