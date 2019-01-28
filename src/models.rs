use crate::schema::links;

// #[derive(Queryable)]
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub password: String,
//     pub email: String,
// }

// #[derive(Insertable)]
// #[table_name="users"]
// pub struct NewUser {
//     pub username: String,
//     pub password: String,
//     pub email: String,
// }

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    // pub owner: Option<i32>,
    pub origin: String,
    pub dest: String,
    // pub is_private: bool,
    pub clicks: i32,
    // lifespan: i32,
    // expiration_date:
    // created_date:
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    // pub owner: Option<i32>,
    pub origin: String,
    pub dest: String,
    // pub is_private: bool,
    // lifespan: i32,
    // expiration_date:
    // created_date:
}
