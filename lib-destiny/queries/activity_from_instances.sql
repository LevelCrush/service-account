SELECT
    COALESCE(activities.name, 'Classified') AS activity_name,
    COALESCE(activities.description, 'N/A') AS activity_description,
    instances.activity_hash AS activity_hash,
    COALESCE(director_activity.name, 'Classified') as director_activity_name,
    COALESCE(director_activity.description,'N/A') AS director_activity_description,
    instances.activity_director_hash AS director_activity_hash,
    COUNT(DISTINCT instances.instance_id) AS total,
    SUM(instances.completed) AS total_completed
FROM instances
LEFT JOIN manifest_activities AS activities ON instances.activity_hash = activities.hash
LEFT JOIN manifest_activities AS director_activity ON instances.activity_director_hash = director_activity.hash
WHERE instances.instance_id IN ({})
GROUP BY  instances.activity_hash, instances.activity_director_hash