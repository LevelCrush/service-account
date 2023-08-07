UPDATE clan_members
SET
    group_id = ?,
    group_role = ?,
    membership_id = ?,
    platform = ?,
    joined_at = ?,
    updated_at = ?,
    deleted_at = ?
WHERE id = ?