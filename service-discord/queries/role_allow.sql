DELETE FROM role_denies 
WHERE role_denies.guild_id = ? 
AND role_denies.role_name = ? 
AND role_denies.member_id = ?