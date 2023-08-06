SELECT
    *
FROM member_triumphs
WHERE member_triumphs.membership_id = ?
AND member_triumphs.hash IN ({})