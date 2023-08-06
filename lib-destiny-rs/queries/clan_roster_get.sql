SELECT
    members.membership_id,
    members.platform,
    members.last_played_at,
    members.display_name,
    members.display_name_global,
    members.updated_at,
    clans.group_id AS clan_group_id,
    clans.name AS clan_name,
    clans.call_sign AS clan_call_sign,
    clan_members.joined_at AS clan_joined_at,
    clan_members.group_role AS clan_group_role,
    clans.is_network AS clan_is_network
FROM clan_members
INNER JOIN clans ON clan_members.group_id = clans.group_id
INNER JOIN members ON clan_members.membership_id = members.membership_id
WHERE clans.group_id = ? 
ORDER BY  clan_members.group_role DESC, members.display_name_global ASC