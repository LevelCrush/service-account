    INSERT INTO classes (
        `id`,
        `hash`,
        `index`,
        `type`,
        `name`,
        `created_at`,
        `updated_at`,
        `deleted_at`
    )
    VALUES {} 
    ON DUPLICATE KEY UPDATE
        `name` = VALUES(`name`),
        `type` = VALUES(`type`),
        `updated_at` = VALUES(`created_at`),
        `deleted_at` = VALUES(`deleted_at`)