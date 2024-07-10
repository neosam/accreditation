-- Your SQL goes here
create table attendant (
    id uuid primary key,
    member_id uuid not null unique,
    registered_at timestamp with time zone not null,
    FOREIGN KEY (member_id) REFERENCES member(id)
)