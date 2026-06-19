CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY, 
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    name VARCHAR(128) NOT NULL,
    email VARCHAR(128) NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS application (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    modified_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    company_name VARCHAR(255) NOT NULL,
    job_title VARCHAR(255) NOT NULL,
    company_location VARCHAR(255),
    job_link TEXT,
    employment_type VARCHAR(50),
    status VARCHAR(50) NOT NULL DEFAULT 'applied',
    applied_date DATE NOT NULL,
    notes TEXT
);