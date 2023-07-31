INSERT INTO manifest_seasons (
    `hash`,
    `name`,
    `pass_hash`,
    `number`,
    `starts_at`,
    `ends_at`,
    `created_at`,
    `updated_at`,
    `deleted_at`
)
VALUES {}
ON CONFLICT(`hash`) 
DO UPDATE SET 
    `name` = excluded.`name`,
    `starts_at` = excluded.`starts_at`,
    `ends_at` = excluded.`ends_at`,
    `updated_at` = excluded.`created_at`,
    `deleted_at` = excluded.`deleted_at`