INSERT INTO member_activity_stats (
    `id`,
    `membership_id`,
    `character_id`,
    `instance_id`,
    `name`,
    `value`,
    `value_display`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON DUPLICATE KEY UPDATE 
    `value` = VALUES(`value`),
    `value_display` = VALUES(`value_display`),
    `updated_at` = VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)