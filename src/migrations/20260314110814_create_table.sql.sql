-- Add migration script here
CREATE TABLE IF NOT EXISTS resources (
    id SERIAL PRIMARY KEY,
    name TEXT
);
CREATE TABLE IF NOT EXISTS outbox (
    id SERIAL PRIMARY KEY,
    msg TEXT
);