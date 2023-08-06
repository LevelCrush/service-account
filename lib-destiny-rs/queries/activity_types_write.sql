INSERT INTO manifest_activity_types (`hash`, `name`, `description`, `icon_url`, `index`,`created_at`, `updated_at`, `deleted_at`)
VALUES {}
ON CONFLICT(`hash`) 
DO UPDATE SET 
    `name` = excluded.`name`,
    `description` = excluded.`description`,
    `icon_url` = excluded.`icon_url`,
    `index` = excluded.`index`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`