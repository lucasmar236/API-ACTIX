-- Your SQL goes here
CREATE TABLE "users"(
    id serial primary key ,
    first_name Varchar not null ,
    last_name Varchar not null ,
    password Varchar not null ,
    email Varchar not null
);