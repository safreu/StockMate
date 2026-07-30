-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE households (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE household_members (
    household_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (household_id, user_id),

    CONSTRAINT household_members_household_fk
        FOREIGN KEY (household_id)
        REFERENCES households (id)
        ON DELETE CASCADE,

    CONSTRAINT household_members_users_fk
        FOREIGN KEY (household_id)
        REFERENCES users (id)
        ON DELETE CASCADE,

    CONSTRAINT household_members_role_check
        CHECK (role IN ('owner', 'member'))
);
