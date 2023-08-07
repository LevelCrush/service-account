UPDATE members 
SET
    platform = ?,
    display_name = ?,
    display_name_global = ?,
    guardian_rank_current = ?,
    guardian_rank_lifetime = ?,
    last_played_at = ?,
    updated_at = ? 
WHERE members.id = ?