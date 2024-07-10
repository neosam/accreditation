-- Your SQL goes here
create table member (
    id uuid primary key,
    first_name varchar(255) not null,
    last_name varchar(255) not null,
    member_id integer not null unique
)