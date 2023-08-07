INSERT INTO manifest_classes (
    `hash`,
    `index`,
    `type`,
    `name`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {} 
ON CONFLICT(`hash`) 
DO UPDATE SET 
    `name` = excluded.`name`,
    `type` = excluded.`type`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`