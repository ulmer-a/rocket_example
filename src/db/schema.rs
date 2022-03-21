use diesel::*;

table! {
    users(id) {
        id -> Nullable<Integer>,
        username -> VarChar,
        password_hash -> Nullable<VarChar>,
    }
}
