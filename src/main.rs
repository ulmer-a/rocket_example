#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
use diesel::{Queryable, RunQueryDsl};
use rocket_sync_db_pools::{database};

mod schema {
    use diesel::*;

    table! {
        users(id) {
            id -> Integer,
            username -> VarChar,
        }
    }
}

use schema::users;
use rocket::{routes, fs::NamedFile, response::status::NotFound, form::Form, fairing::AdHoc, Rocket, Build};

#[derive(Queryable)]
struct User {
    id: i32,
    username: String,
}

#[get("/")]
async fn index(conn: MainDatabase) -> String {
    let mut usernames = String::new();
    for user in conn.run(|c| {
        users::dsl::users.load::<User>(c)
    }).await.unwrap() {
        usernames += &format!("{}, ", user.username);
    }
    usernames
}

#[get("/login")]
async fn serve_login() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("login.html").await.map_err(|e| NotFound(e.to_string()))
}

#[derive(FromForm)]
struct Credentials {
    user: String,
    pass: String
}

#[post("/login", data="<credentials>")]
fn login(credentials: Form<Credentials>) -> String {
    format!("Hello {}, your password is: '{}'", credentials.user, credentials.pass)
}

#[database("main_db")]
struct MainDatabase(diesel::SqliteConnection);

async fn db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = MainDatabase::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("can run migrations");

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDatabase::fairing())
        .attach(AdHoc::on_ignite("DB Migrations", db_migrations))
        .mount("/", routes![index, serve_login, login])
}
