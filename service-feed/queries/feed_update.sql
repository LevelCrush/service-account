UPDATE feeds
SET 
    feeds.data = ?,
    feeds.updated_at = ?
WHERE feeds.id = ?