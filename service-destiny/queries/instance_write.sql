INSERT INTO instances (
    `instance_id`,
    `occurred_at`,
    `starting_phase_index`,
    `started_from_beginning`,
    `activity_hash`,
    `activity_director_hash`,
    `is_private`,
    `completed`,
    `completion_reasons`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES (?,?,?,?,?,?,?,?,?,?,?,?)
ON CONFLICT(`instance_id`)
DO UPDATE SET
    `occurred_at` = excluded.`occurred_at`,
    `starting_phase_index` = excluded.`starting_phase_index`,
    `started_from_beginning` = excluded.`started_from_beginning`,
    `activity_hash` = excluded.`activity_hash`,
    `activity_director_hash` = excluded.`activity_director_hash`,
    `is_private` = excluded.`is_private`,
    `completed` = excluded.`completed`,
    `completion_reasons` = excluded.`completion_reasons`,
    `updated_at` =  excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`