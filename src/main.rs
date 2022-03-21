mod db;
mod login;

use rocket::{fairing::AdHoc, routes};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// #[get("/")]
// async fn index(conn: MainDatabase) -> String {
//     let mut usernames = String::new();
//     for user in conn.run(|c| {
//         users::dsl::users.load::<User>(c)
//     }).await.unwrap() {
//         usernames += &format!("{}, ", user.username);
//     }
//     usernames
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::MainDatabase::fairing())
        .attach(AdHoc::on_ignite("DB Migrations", db::migrations))
        .mount("/", routes![login::base, login::form])
}
