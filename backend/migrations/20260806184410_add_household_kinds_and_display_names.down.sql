-- Add down migration script here
DROP INDEX IF EXISTS household_members_user_id_idx;
DROP INDEX IF EXISTS households_personal_owner_unique_idx;

ALTER TABLE households
    DROP CONSTRAINT IF EXISTS households_kind_owner_check,
    DROP CONSTRAINT IF EXISTS households_personal_owner_fk,
    DROP CONSTRAINT IF EXISTS households_kind_check,
    DROP CONSTRAINT IF EXISTS households_name_length,
    DROP CONSTRAINT IF EXISTS households_name_not_empty;

ALTER TABLE households
    DROP COLUMN IF EXISTS personal_owner_id,
    DROP COLUMN IF EXISTS kind;

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_display_name_length,
    DROP CONSTRAINT IF EXISTS users_display_name_not_empty;

ALTER TABLE users
    DROP COLUMN IF EXISTS display_name;