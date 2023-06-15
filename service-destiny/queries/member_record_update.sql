UPDATE members 
SET
    members.platform = ?,
    members.display_name = ?,
    members.display_name_global = ?,
    members.guardian_rank_current = ?,
    members.guardian_rank_lifetime = ?,
    members.last_played_at = ?,
    members.updated_at = ? 
WHERE members.id = ?