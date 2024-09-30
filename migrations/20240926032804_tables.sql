-- Add migration script here
-- Ensure the UUID extension is available for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,  -- Store hashed passwords
    public_key TEXT,             -- Store the user's public key
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Files table
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,  -- Foreign key to users table
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    encrypted_aes_key BYTEA NOT NULL,  -- Store encrypted AES key
    encrypted_file BYTEA NOT NULL,     -- Store the actual encrypted file content
    iv BYTEA NOT NULL,                 -- Initialization vector for AES encryption
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Shared links table (with required password and expiration date)
CREATE TABLE shared_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_id UUID REFERENCES files(id) ON DELETE CASCADE,  -- Foreign key to files table
    recipient_user_id UUID REFERENCES users(id) ON DELETE CASCADE,  -- Foreign key to users table
    password VARCHAR(255) NOT NULL,  -- Password protection (required)
    expiration_date TIMESTAMP WITH TIME ZONE NOT NULL,  -- Expiration date (required)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);