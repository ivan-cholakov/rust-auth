-- migrations/20240101000000_add_password_hash.sql
ALTER TABLE users ADD COLUMN password_hash VARCHAR(255) NOT NULL;