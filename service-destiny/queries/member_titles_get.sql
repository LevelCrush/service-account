WITH
target_member AS (
    SELECT members.*
    FROM members
    WHERE members.membership_id  = ?
),
seals AS (
    SELECT
        triumphs.hash,
        triumphs.title,
        triumphs.gilded
    FROM triumphs
    WHERE triumphs.is_title = 1
), # grab all seals (titles)

member_seals_gilded AS (
    SELECT
        target_member.membership_id,
        seals.title,
        member_triumphs.state,
        member_triumphs.times_completed,
        member_triumphs.state & 64 = 64 AS can_equip
    FROM member_triumphs
    INNER JOIN seals ON member_triumphs.hash = seals.hash
    INNER JOIN target_member ON member_triumphs.membership_id = target_member.membership_id
    AND seals.gilded = 1 # looking for seals that have a gilded version
    AND member_triumphs.times_completed > 0 # and our member has completed it > 0 which means they have gilded at some point
),
member_seals AS (
    SELECT
        target_member.membership_id,
        seals.title,
        member_triumphs.state,
        member_triumphs.times_completed,
        member_triumphs.state & 64 = 64 AS can_equip
    FROM member_triumphs
    INNER JOIN seals ON member_triumphs.hash = seals.hash
    INNER JOIN target_member ON member_triumphs.membership_id = target_member.membership_id
    AND seals.gilded = 0 # looking for seals that dont have a gilded version
)
# The below will merge all the gilded versions with the non gilded versions by Title.
# There can be multiple versions of the same title, they just have slightly different objectives but they are still the same Title regardless
SELECT
    member_seals.membership_id,
    member_seals.title,
    COALESCE(member_seals_gilded.times_completed > 0, 0) AS has_gilded,
    COALESCE(member_seals_gilded.times_completed, 0) AS total_gilds,
    member_seals.can_equip,
    COALESCE(member_seals_gilded.can_equip, 0) AS can_equip_gilded
FROM member_seals
LEFT JOIN member_seals_gilded ON member_seals.title = member_seals_gilded.title
WHERE member_seals.can_equip = 1
GROUP BY member_seals.membership_id, member_seals.title, member_seals_gilded.times_completed, member_seals_gilded.can_equip
ORDER BY member_seals.title ASC