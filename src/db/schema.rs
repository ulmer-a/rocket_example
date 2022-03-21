use diesel::*;

table! {
    users(id) {
        id -> Integer,
        username -> VarChar,
        password_hash -> Nullable<VarChar>,
    }
}
