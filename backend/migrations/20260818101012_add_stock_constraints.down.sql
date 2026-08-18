-- Add down migration script here
SET lock_timeout = '5s';

ALTER TABLE inventory_items
    DROP CONSTRAINT inventory_items_current_stock_u32_max,
    DROP CONSTRAINT inventory_items_reoder_threshold_u32_max;