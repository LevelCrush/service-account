INSERT INTO instance_members (
    `id`,
    `instance_id`,
    `membership_id`,
    `platform`,
    `character_id`,
    `class_hash`,
    `class_name`,
    `emblem_hash`,
    `light_level`,
    `clan_name`,
    `clan_tag`,
    `completed`,
    `completion_reason`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON DUPLICATE KEY UPDATE
    `class_hash` = VALUES(`class_hash`),
    `class_name` = VALUES(`class_name`),
    `emblem_hash` = VALUES(`emblem_hash`),
    `light_level` = VALUES(`light_level`),
    `clan_name` = VALUES(`clan_name`),
    `clan_tag` = VALUES(`clan_tag`),
    `completed` = VALUES(`completed`),
    `completion_reason` = VALUES(`completion_reason`),
    `updated_at` = VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)