INSERT INTO account_platform_data 
(`id`, `account`, `platform`, `key`, `value`, `created_at`, `updated_at`, `deleted_at`)
VALUES {}
ON DUPLICATE KEY UPDATE
    `value` = VALUES(`value`),
    `updated_at` = VALUES(`updated_at`),
    `deleted_at` = VALUES(`deleted_at`)