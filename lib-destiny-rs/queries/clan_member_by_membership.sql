SELECT
    clan_members.id,
    clan_members.membership_id
FROM clan_members
WHERE clan_members.membership_id IN ({})
ORDER BY clan_members.id ASC