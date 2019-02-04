-- Your SQL goes here
CREATE TABLE people (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION hash_password() RETURNS TRIGGER AS $$
BEGIN
  IF NEW.password IS NOT NULL AND ( TG_OP = 'INSERT' OR NEW.password != OLD.password ) THEN
	  NEW.password = crypt(NEW.password, gen_salt('bf', 8));
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tg_hash_password BEFORE INSERT OR UPDATE ON people
FOR EACH ROW EXECUTE PROCEDURE hash_password();
