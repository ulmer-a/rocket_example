use super::db;
use rocket::{form::Form, fs::NamedFile, response::status::NotFound};

use pbkdf2::{Pbkdf2, password_hash::{PasswordVerifier, PasswordHash}};

#[get("/login")]
pub async fn base() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("login.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[derive(FromForm)]
pub struct Credentials {
    user: String,
    pass: String,
}

#[post("/login", data = "<credentials>")]
pub async fn form(credentials: Form<Credentials>, conn: db::MainDatabase) -> String {
    if let Some(user) = db::find_user_by_name(credentials.user.clone(), &conn).await {
        if let Some(password_hash) = user.password_hash {
            let parsed_hash = PasswordHash::new(&password_hash).unwrap(); // TODO: log and return internal server error
            if Pbkdf2.verify_password(&credentials.pass.as_bytes(), &parsed_hash).is_ok() {
                format!("user {} with id {} successfully logged in!", user.username, user.id)
            } else {
                format!("invalid password for user '{}'", credentials.user)
            }
        } else {
            format!("user '{}' hasn't got a password set!", user.username)
        }
    } else {
        format!("user '{}' not known!", credentials.user)
    }
}
