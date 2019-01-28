-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS tg_hash_password ON users;
DROP FUNCTION IF EXISTS hash_password();
DROP TABLE IF EXISTS users;
