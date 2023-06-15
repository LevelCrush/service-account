INSERT INTO account_platform_data 
(`id`, `account`, `platform`, `key`, `value`, `value_bigint`, `value_big`, `created_at`, `updated_at`, `deleted_at`)
VALUES {}
ON DUPLICATE KEY UPDATE
    `value` = VALUES(`value`),
    `value_big` = VALUES(`value_big`),
    `value_bigint` = VALUES(`value_bigint`),
    `updated_at` = VALUES(`updated_at`),
    `deleted_at` = VALUES(`deleted_at`)