CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR
);