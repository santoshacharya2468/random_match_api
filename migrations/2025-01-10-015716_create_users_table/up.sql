-- Enable the uuid-ossp extension (for PostgreSQL)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the table with id as UUID
CREATE TABLE auth_users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT,
    username TEXT NOT NULL UNIQUE ,
    password TEXT,
    provider TEXT NOT NULL,
    external_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

