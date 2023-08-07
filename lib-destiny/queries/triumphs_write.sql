INSERT INTO manifest_triumphs (
    `hash`,
    `name`,
    `description`,
    `title`,
    `is_title`,
    `gilded`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON CONFLICT(`hash`) 
DO UPDATE SET 
    `name` = excluded.`name`,
    `description` = excluded.`description`,
    `title` = excluded.`title`,
    `is_title` = excluded.`is_title`,
    `gilded` = excluded.`gilded`,
    `updated_at` = excluded.`created_at`