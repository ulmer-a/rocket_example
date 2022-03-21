mod db;
mod login;

use rocket::{fairing::AdHoc, response::Redirect, routes};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[get("/")]
async fn index() -> Redirect {
    Redirect::to(format!("/login"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::MainDatabase::fairing())
        .attach(AdHoc::on_ignite("DB Migrations", db::migrations))
        .mount(
            "/",
            routes![index, login::base, login::form, login::register],
        )
}
