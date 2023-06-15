SELECT 
    COALESCE(MAX(member_activities.occurred_at), 0) AS  timestamp
FROM member_activities 
WHERE member_activities.character_id = ?
LIMIT 1