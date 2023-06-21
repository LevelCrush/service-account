INSERT INTO seasons (
    `id`,
    `hash`,
    `name`,
    `pass_hash`,
    `number`,
    `starts_at`,
    `ends_at`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON DUPLICATE KEY UPDATE 
    `name` = VALUES(`name`),
    `starts_at` = VALUES(`starts_at`),
    `ends_at` = VALUES(`ends_at`),
    `updated_at` = VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)