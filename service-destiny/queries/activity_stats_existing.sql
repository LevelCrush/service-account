SELECT 
    member_activity_stats.id,
    member_activity_stats.instance_id
FROM member_activity_stats
WHERE member_activity_stats.character_id = ?
AND member_activity_stats.instance_id IN({})