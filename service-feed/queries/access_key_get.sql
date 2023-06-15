SELECT
    access_keys.*
FROM access_keys
WHERE access_keys.public_key = ?
AND access_keys.private_key = ?
AND access_keys.deleted_at = 0
LIMIT 1