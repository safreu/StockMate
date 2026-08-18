-- Add up migration script here
SET lock_timeout = '5s';

ALTER TABLE inventory_items
    ADD CONSTRAINT inventory_items_current_stock_u32_max
        CHECK (current_stock <= 4294967295),

    ADD CONSTRAINT inventory_items_reoder_threshold_u32_max
    CHECK (reorder_threshold <= 4294967295);