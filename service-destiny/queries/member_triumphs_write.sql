INSERT INTO member_triumphs (
    `hash`,
    `membership_id`,
    `state`,
    `times_completed`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON CONFLICT(`hash`,`membership_id`)
DO UPDATE SET 
    `state` = excluded.`state`,
    `times_completed` = excluded.`times_completed`,
    `updated_at` = excluded.`created_at`