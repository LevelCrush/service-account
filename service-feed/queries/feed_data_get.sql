SELECT 
    feeds.data
FROM feeds
INNER JOIN access_keys ON access_keys.public_key = ?
WHERE feeds.deleted_at = 0
AND feeds.slug  = ? 
LIMIT 1