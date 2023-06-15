WITH
target_activities  AS
(
    SELECT DISTINCT member_activities.instance_id FROM member_activities
    WHERE (member_activities.occurred_at > ? AND member_activities.occurred_at < ?)
),
instance_member_count AS (
    SELECT
        target_activities.instance_id,
        COUNT(instance_members.id) AS instance_members
    FROM target_activities
    LEFT JOIN instance_members ON target_activities.instance_id = instance_members.instance_id
    GROUP BY target_activities.instance_id
)
SELECT
    target_activities.instance_id
FROM target_activities
INNER JOIN instance_member_count ON target_activities.instance_id = instance_member_count.instance_id
WHERE instance_member_count.instance_members = 0
LIMIT ?