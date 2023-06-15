SELECT
    members.membership_id,
    members.platform,
    members.last_played_at,
    members.display_name,
    members.display_name_global,
    members.updated_at,
    COALESCE(clans.group_id, 0) AS clan_group_id,
    COALESCE(clans.name, 0) AS clan_name,
    COALESCE(clans.call_sign, 0) AS clan_call_sign,
    COALESCE(clan_members.joined_at,0) AS clan_joined_at,
    COALESCE(clan_members.group_role,0) AS clan_group_role,
    COALESCE(clans.is_network,0) AS clan_is_network
FROM members
LEFT JOIN clan_members ON members.membership_id = clan_members.membership_id
LEFT JOIN clans ON clan_members.group_id = clans.group_id
WHERE members.membership_id = ?
LIMIT 1