-- Your SQL goes here
CREATE TYPE client_categories AS ENUM('privileged', 'public');

CREATE TABLE clients (
    id uuid DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    secret TEXT NOT NULL,
    category client_categories NOT NULL DEFAULT 'public',
    owner uuid NOT NULL REFERENCES people(id),
    PRIMARY KEY (id)
);
