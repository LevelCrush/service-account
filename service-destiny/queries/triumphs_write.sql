INSERT INTO triumphs (
    `id`,
    `hash`,
    `name`,
    `description`,
    `title`,
    `is_title`,
    `gilded`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON DUPLICATE KEY UPDATE
    `name` = VALUES(`name`),
    `description` = VALUES(`description`),
    `title` = VALUES(`title`),
    `is_title` = VALUES(`is_title`),
    `gilded` = VALUES(`gilded`),
    `updated_at` = VALUES(`created_at`)