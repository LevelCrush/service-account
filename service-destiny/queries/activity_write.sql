INSERT INTO manifest_activities (
    `hash`, 
    `index`, 
    `activity_type`,
    `name`, 
    `description`, 
    `image_url`, 
    `fireteam_min_size`,
    `fireteam_max_size`,
    `max_players`,
    `requires_guardian_oath`,
    `is_pvp`,
    `matchmaking_enabled`,
    `created_at`,
    `updated_at`, 
    `deleted_at`
)
VALUES {}
ON CONFLICT(`hash`) 
DO UPDATE SET 
    `name` = excluded.`name`,
    `description` = excluded.`description`,
    `image_url` = excluded.`image_url`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`,
    `index` = excluded.`index`