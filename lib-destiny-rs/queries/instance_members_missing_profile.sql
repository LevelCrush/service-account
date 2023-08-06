SELECT
    instance_members.membership_id,
    instance_members.platform,
    instances.occurred_at AS timestamp
FROM instances
INNER JOIN instance_members ON instances.instance_id = instance_members.instance_id
LEFT JOIN members ON instance_members.membership_id = members.membership_id
WHERE members.id IS NULL
AND (instances.occurred_at > ? AND instances.occurred_at < ?)
GROUP BY instance_members.membership_id, instance_members.platform, instances.occurred_at
LIMIT ?