SELECT
    COUNT(DISTINCT members.id) AS total
FROM members
LEFT JOIN clan_members ON members.membership_id = clan_members.membership_id
LEFT JOIN clans ON clan_members.group_id = clans.group_id
WHERE
(
    members.display_name LIKE ? OR
    members.display_name_global LIKE ?
)
AND members.deleted_at = 0