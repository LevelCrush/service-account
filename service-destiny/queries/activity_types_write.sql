INSERT INTO activity_types (`id`, `hash`, `name`, `description`, `icon_url`, `index`,`created_at`, `updated_at`, `deleted_at`)
VALUES {}
ON DUPLICATE KEY UPDATE
    `name` = VALUES(`name`),
    `description` = VALUES(`description`),
    `icon_url` = VALUES(`icon_url`),
    `index` = VALUES(`index`),
    `updated_at` = VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)