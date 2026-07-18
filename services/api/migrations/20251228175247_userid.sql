-- Add migration script here
-- Add migration script here
ALTER TABLE products
ADD COLUMN user_id UUID REFERENCES users(id);