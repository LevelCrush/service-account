INSERT INTO role_denies
SET 
    `id` = 0,
    `guild_id` = ?,
    `role_name` = ?,
    `member_id` = ?,
    `created_at` = ?,
    `updated_at` = 0,
    `deleted_at` = 0
ON DUPLICATE KEY UPDATE 
    `updated_at` = VALUES(`created_at`)