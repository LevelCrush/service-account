SELECT 
    activities.id,
    activities.hash
FROM manifest_activities AS activities
WHERE activities.hash IN ({})