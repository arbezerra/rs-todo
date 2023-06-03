-- Add up migration script here
create table user (
    id binary(16) default(uuid_to_bin(uuid())) not null primary key,
    name varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null
);