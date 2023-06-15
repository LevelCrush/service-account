INSERT INTO instances (
    `id`,
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
VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)
ON DUPLICATE KEY UPDATE 
    `occurred_at` = VALUES(`occurred_at`),
    `starting_phase_index` = VALUES(`starting_phase_index`),
    `started_from_beginning` = VALUES(`started_from_beginning`),
    `activity_hash` = VALUES(`activity_hash`),
    `activity_director_hash` = VALUES(`activity_director_hash`),
    `is_private` = VALUES(`is_private`),
    `completed` = VALUES(`completed`),
    `completion_reasons` = VALUES(`completion_reasons`),
    `updated_at` =  VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)