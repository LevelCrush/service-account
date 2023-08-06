SELECT 
    *
FROM manifest_seasons as seasons
WHERE seasons.starts_at > 0 
AND seasons.ends_at > 0 
AND seasons.name NOT LIKE '%[Redacted]'
ORDER BY seasons.number DESC