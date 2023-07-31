INSERT INTO member_activity_stats (

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
ON CONFLICT(`membership_id`,`character_id`,`instance_id`) 
DO UPDATE SET
    `value` = excluded.`value`,
    `value_display` = excluded.`value_display`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`