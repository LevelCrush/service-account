SELECT
    feeds.id
FROM feeds
WHERE feeds.access_key = ? 
AND feeds.slug = ?
AND feeds.deleted_at = 0
LIMIT 1