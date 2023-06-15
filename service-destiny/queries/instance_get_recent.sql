SELECT
    instances.* 
FROM instances
ORDER BY instances.occurred_at DESC
LIMIT 1