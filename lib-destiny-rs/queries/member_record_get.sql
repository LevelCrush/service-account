SELECT
    members.*
FROM members
WHERE members.membership_id = ?
LIMIT 1