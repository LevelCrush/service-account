SELECT
    activity_types.id,
    activity_types.hash
FROM activity_types
WHERE activity_types.hash IN ({})