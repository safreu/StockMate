-- Add up migration script here
CREATE TABLE categories (
    id UUID PRIMARY KEY,
    household_id UUID NOT NULL,
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT categories_household_fk
        FOREIGN KEY (household_id)
        REFERENCES households (id)
        ON DELETE CASCADE,

    CONSTRAINT categories_household_name_unique
        UNIQUE (household_id, normalized_name)
);

CREATE TABLE inventory_items (
    id UUID PRIMARY KEY,
    household_id UUID NOT NULL,
    category_id UUID,
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,

    current_stock BIGINT NOT NULL,
    reorder_threshold BIGINT NOT NULL,

    priority TEXT NOT NULL DEFAULT 'default',

    archived_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT inventory_items_household_fk
        FOREIGN KEY (household_id)
        REFERENCES households (id)
        ON DELETE CASCADE,

    CONSTRAINT inventory_items_category_fk
        FOREIGN KEY (category_id)
        REFERENCES categories (id)
        ON DELETE SET NULL,

    CONSTRAINT inventory_items_current_stock_non_negative
        CHECK (current_stock >= 0),

    CONSTRAINT inventory_items_reorder_threshold_non_negative
        CHECK (reorder_threshold >= 0),

    CONSTRAINT inventory_items_priority_valid
        CHECK (
            priority IN (
                'default',
                'low',
                'medium',
                'high'
            )
        )
);

CREATE UNIQUE INDEX inventory_items_active_name_unique_idx
    ON inventory_items (
        household_id,
        normalized_name
    )
    WHERE archived_at IS NULL;

CREATE INDEX categories_household_id_idx
    ON categories (household_id);

CREATE INDEX inventory_items_household_id_idx
    ON inventory_items (household_id);

CREATE INDEX inventory_items_category_id_idx
    ON inventory_items (category_id);

CREATE INDEX inventory_items_active_household_idx
    ON inventory_items (household_id)
    WHERE archived_at IS NULL;