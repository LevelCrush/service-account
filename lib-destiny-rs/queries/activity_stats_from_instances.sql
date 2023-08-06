SELECT
    member_activity_stats.membership_id,
    member_activity_stats.instance_id,
    member_activity_stats.value,
    member_activity_stats.value_display
FROM member_activity_stats
WHERE member_activity_stats.name = ?
AND member_activity_stats.membership_id = ?
{}
AND member_activity_stats.instance_id IN ({})
GROUP BY 
    member_activity_stats.membership_id, member_activity_stats.instance_id, 
    member_activity_stats.value, member_activity_stats.value_display