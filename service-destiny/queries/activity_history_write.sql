INSERT INTO member_activities 
(
    `id`,
    `membership_id`,
    `character_id`,
    `platform_played`,
    `activity_hash`,
    `activity_hash_director`,
    `instance_id`,
    `mode`,
    `modes`,
    `private`,
    `occurred_at`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON DUPLICATE KEY UPDATE
    `platform_played` = VALUES(`platform_played`),
    `activity_hash` = VALUES(`activity_hash`),
    `activity_hash_director` = VALUES(`activity_hash_director`),
    `mode` = VALUES(`mode`),
    `modes` = VALUES(`modes`),
    `occurred_at` = VALUES(`occurred_at`),
    `updated_at` = VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)