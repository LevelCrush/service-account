SELECT 
    activities.id,
    activities.hash
FROM activities
WHERE activities.hash IN ({})