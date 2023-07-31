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
ON CONFLICT(`membership_id`, `character_id`,`instance_id`)
DO UPDATE SET
    `platform_played` = excluded.`platform_played`,
    `activity_hash` = excluded.`activity_hash`,
    `activity_hash_director` = excluded.`activity_hash_director`,
    `mode` = excluded.`mode`,
    `modes` = excluded.`modes`,
    `occurred_at` =  excluded.`occurred_at`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`