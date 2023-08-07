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
INNER JOIN clan_members AS target_member ON clans.group_id = target_member.group_id
LEFT JOIN clan_members ON clans.group_id = clan_members.group_id
WHERE target_member.membership_id = ?
GROUP BY clans.group_id
LIMIT 1