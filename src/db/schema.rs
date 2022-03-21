use diesel::*;

table! {
    users(id) {
        id -> Integer,
        username -> VarChar,
    }
}
