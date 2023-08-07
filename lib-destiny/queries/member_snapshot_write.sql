INSERT INTO member_snapshots
(
    `membership_id`,
    `snapshot_name`,
    `version`,
    `data`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES (?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(`membership_id`,`snapshot_name`,`version`)
DO UPDATE SET 
    `data` = excluded.`data`,
    `updated_at` = excluded.created_at