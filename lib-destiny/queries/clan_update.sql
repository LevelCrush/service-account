UPDATE clans
SET
    group_id = ?,
    name = ?,
    slug = ?,
    motto = ?,
    about = ?,
    call_sign = ?,
    updated_at = ?,
    deleted_at = ?
WHERE clans.id = ?