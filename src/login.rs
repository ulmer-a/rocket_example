use super::db;
use rocket::{form::Form, fs::NamedFile, response::status::NotFound};

#[get("/login")]
pub async fn base() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("login.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(FromForm)]
pub struct Credentials {
    user: String,
    _pass: String,
}

#[post("/login", data = "<credentials>")]
pub async fn form(credentials: Form<Credentials>, conn: db::MainDatabase) -> String {
    if let Some(user) = db::find_user_by_name(credentials.user.clone(), &conn).await {
        format!("logged in using user {} with id {}", user.username, user.id)
    } else {
        format!("username not known!")
    }
}
