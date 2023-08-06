SELECT
    *
FROM member_activities
INNER JOIN members ON member_activities.membership_id = members.membership_id 
WHERE members.membership_id = ?
AND (member_activities.occurred_at >= ? AND member_activities.occurred_at <= ?)
{}
ORDER BY member_activities.occurred_at DESC