INSERT INTO member_triumphs (
    `id`,
    `hash`,
    `membership_id`,
    `state`,
    `times_completed`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON DUPLICATE KEY UPDATE
    `state` = VALUES(`state`),
    `times_completed` = VALUES(`times_completed`),
    `updated_at` = VALUES(`created_at`)