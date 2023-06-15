SELECT
    clans.*
FROM clans
WHERE clans.group_id = ?
LIMIT 1