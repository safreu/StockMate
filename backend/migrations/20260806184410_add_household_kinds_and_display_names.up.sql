-- Add up migration script here
ALTER TABLE users
    ADD COLUMN display_name TEXT;

UPDATE users
SET display_name = COALESCE(
    NULLIF(BTRIM(split_part(email, '@', 1)), ''),
    'User'
)
WHERE display_name IS NULL;

ALTER TABLE users
    ALTER COLUMN display_name SET NOT NULL;

ALTER TABLE users
    ADD CONSTRAINT users_display_name_not_empty
        CHECK (BTRIM(display_name) <> ''),

    ADD CONSTRAINT users_display_name_length
        CHECK (CHAR_LENGTH(display_name) <= 100);

ALTER TABLE households
    ADD COLUMN kind TEXT,
    ADD COLUMN personal_owner_id UUID;

UPDATE households
SET kind = 'shared'
WHERE kind IS NULL;

ALTER TABLE households
    ALTER COLUMN kind SET NOT NULL;

ALTER TABLE households
    ADD CONSTRAINT households_name_not_empty
        CHECK (BTRIM(name) <> ''),

    ADD CONSTRAINT households_name_length
        CHECK (CHAR_LENGTH(name) <= 100),

    ADD CONSTRAINT households_kind_check
        CHECK (kind IN ('personal', 'shared')),

    ADD CONSTRAINT households_personal_owner_fk
        FOREIGN KEY (personal_owner_id)
        REFERENCES users (id)
        ON DELETE CASCADE,

    ADD CONSTRAINT households_kind_owner_check
        CHECK (
            (kind = 'personal' AND personal_owner_id IS NOT NULL)
            OR
            (kind = 'shared' AND personal_owner_id IS NULL)
        );

CREATE UNIQUE INDEX households_personal_owner_unique_idx
    ON households (personal_owner_id)
    WHERE personal_owner_id IS NOT NULL;

CREATE INDEX household_members_user_id_idx
    ON household_members (user_id);