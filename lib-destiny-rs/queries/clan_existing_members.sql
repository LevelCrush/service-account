SELECT
    clan_members.id,
    clan_members.membership_id
FROM clan_members
WHERE clan_members.group_id = ?