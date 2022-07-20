CREATE TABLE IF NOT EXISTS users (
    id serial primary key not null,
    name varchar not null
);

INSERT INTO users (name) VALUES ('test user 1');
