use crate::schema::links;
use crate::schema::token_key_map;
use chrono::naive::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable)]
pub struct TokenKeyMap {
    pub id: i32,
    pub token: String,
    pub key: String,
}

#[derive(Insertable)]
#[table_name = "token_key_map"]
pub struct NewTokenKeyMap {
    pub token: String,
    pub key: String,
}

#[derive(Queryable, Serialize)]
pub struct Link {
    pub id: i32,
    pub owner: Option<String>,
    pub origin: String,
    pub dest: String,
    pub creation_date: NaiveDateTime,
    pub last_used: Option<NaiveDateTime>,
    pub clicks: i32,
    pub expire_date: Option<NaiveDateTime>,
    pub expire_clicks: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    pub owner: Option<String>,
    pub origin: String,
    pub dest: String,
    pub expire_date: Option<NaiveDateTime>,
    pub expire_clicks: Option<i32>,
}
