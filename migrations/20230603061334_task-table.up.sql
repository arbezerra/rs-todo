-- Add up migration script here
create table task (
    id binary(16) default(uuid_to_bin(uuid())) not null primary key,
    task varchar(255) not null
);