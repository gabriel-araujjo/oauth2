-- Your SQL goes here
CREATE TABLE people (
    id uuid DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    hash TEXT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT people_email_unique UNIQUE (email)
);
