-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(5) NOT NULL,
    password VARCHAR(60) NOT NULL
);

INSERT INTO
    users (name, role, password)
VALUES (
        'admin',
        'admin',
        '$2a$12$l9WF/lJhtrpRB//ymkNmR.IzTkRra8zDkMeDPguKMdjKTDNTq8dBG' -- admin
    );