SELECT
    clans.group_id,
    clans.name,
    clans.slug,
    clans.motto,
    clans.about,
    clans.call_sign,
    clans.is_network,
    clans.updated_at,
    COUNT(DISTINCT clan_members.membership_id) AS member_count
FROM clans
LEFT JOIN clan_members ON clans.group_id = clan_members.group_id
WHERE clans.is_network = 1
GROUP BY clans.group_id, clans.name
ORDER BY clans.name ASC