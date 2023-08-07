SELECT member_activities
    member_activities.id,
    member_activities.instance_id
FROM member_activities
WHERE member_activities.character_id = ?
AND member_activities.instance_id IN ({})