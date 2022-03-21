#[macro_use] extern crate rocket;
use rocket_sync_db_pools::{database, diesel};

use rocket::{routes, fs::NamedFile, response::status::NotFound, form::Form};

#[get("/")]
fn index() -> String {
    format!("youre logged in as Bert")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDatabase::fairing())
        .mount("/", routes![index, serve_login, login])
}
