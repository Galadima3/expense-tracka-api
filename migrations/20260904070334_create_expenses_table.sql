-- Add migration script here
CREATE TYPE categories AS ENUM (
    'food',
    'transport',
    'housing',
    'utilities',
    'entertainment'
);

CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    amount INTEGER NOT NULL CHECK (amount >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    month SMALLINT NOT NULL CHECK (month BETWEEN 1 AND 12),
    category categories NOT NULL
);