use diesel::*;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

pub(crate) mod schema;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

use schema::users::dsl::username as user_field;

pub async fn find_user_by_name(username: String, conn: &MainDatabase) -> Option<User> {
    conn.run(move |c| {
        schema::users::dsl::users
            .filter(user_field.eq(username))
            .first::<User>(c)
            .ok()
    })
    .await
}

pub async fn _find_user_by_id(id: i32, conn: &MainDatabase) -> Option<User> {
    conn.run(move |c| schema::users::dsl::users.find(id).first::<User>(c).ok())
        .await
}

#[database("main_db")]
pub struct MainDatabase(diesel::SqliteConnection);

pub async fn migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = MainDatabase::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("can run migrations");

    rocket
}
