INSERT INTO activities (
    `id`, 
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
ON DUPLICATE KEY UPDATE
    `name` = VALUES(`name`),
    `description` = VALUES(`description`),
    `image_url` = VALUES(`image_url`),
    `updated_at` = VALUES(`created_at`),
    `deleted_at` = VALUES(`deleted_at`)