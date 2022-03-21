use diesel::*;
use rocket::serde::Serialize;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

pub(crate) mod schema;
use schema::users;

#[derive(Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: Option<String>,
}

use users::dsl::username as user_field;

pub async fn find_user_by_name(username: String, conn: &MainDatabase) -> Option<User> {
    conn.run(move |c| {
        users::dsl::users
            .filter(user_field.eq(username))
            .first::<User>(c)
            .ok()
    })
    .await
}

pub async fn _find_user_by_id(id: i32, conn: &MainDatabase) -> Option<User> {
    conn.run(move |c| users::dsl::users.find(id).first::<User>(c).ok())
        .await
}

pub async fn insert_user(username: String, password_hash: String, conn: &MainDatabase) -> bool {
    conn.run(move |c| {
        let user = User {
            id: None,
            username: username,
            password_hash: Some(password_hash)
        };
        if let Err(error) = diesel::insert_into(users::table).values(&user).execute(c) {
            println!("insert user: {}", error);
            false
        } else {
            true
        }
    }).await
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
