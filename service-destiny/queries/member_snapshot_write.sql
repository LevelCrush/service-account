INSERT INTO member_snapshots
(
    `id`,
    `membership_id`,
    `snapshot_name`,
    `version`,
    `data`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES (0, ?, ?, ?, ?, ?, ?, ?)
ON DUPLICATE KEY UPDATE 
    `data` = VALUES(`data`),
    `updated_at` = VALUES(`created_at`)