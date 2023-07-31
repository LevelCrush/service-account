INSERT INTO instance_members (
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
ON CONFLICT(`instance_id`,`membership_id`,`character_id`)
DO UPDATE SET
    `class_hash` = excluded.`class_hash`,
    `class_name` = excluded.`class_name`,
    `emblem_hash` = excluded.`emblem_hash`,
    `light_level` = excluded.`light_level`,
    `clan_name` = excluded.`clan_name`,
    `clan_tag` = excluded.`clan_tag`,
    `completed` = excluded.`completed`,
    `completion_reason` = excluded.`completion_reason`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`