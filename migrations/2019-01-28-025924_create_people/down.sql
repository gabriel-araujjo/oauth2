-- Delete people data
DROP TRIGGER IF EXISTS tg_hash_password ON people;
DROP FUNCTION IF EXISTS hash_password();
DROP TABLE IF EXISTS people;
