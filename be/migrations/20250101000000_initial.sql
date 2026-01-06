CREATE TABLE persons (
    id uuid PRIMARY KEY,
    first_name text NOT NULL,
    middle_name text NULL,
    last_name text NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE person_contacts (
    id uuid PRIMARY KEY,
    person_id uuid REFERENCES persons (id) ON DELETE CASCADE,
    email text UNIQUE NOT NULL,
    phone text UNIQUE,
    created_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE users (
    id uuid PRIMARY KEY,
    user_name text UNIQUE NOT NULL,
    email varchar(255) UNIQUE NOT NULL,
    phone varchar(15) UNIQUE NOT NULL,
    password_hash text NOT NULL,
    person_id uuid UNIQUE REFERENCES persons (id) ON DELETE CASCADE,
    created_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE refresh_tokens (
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES users (id) ON DELETE CASCADE,
    token text NOT NULL,
    expires_at timestamp NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE email_verification_tokens (
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES users (id) ON DELETE CASCADE,
    token text NOT NULL,
    expires_at timestamp NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW()
);

