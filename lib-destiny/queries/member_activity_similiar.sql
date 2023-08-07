WITH
source_member AS (
    SELECT
        members.*
    FROM members
    WHERE members.display_name_global = ?
    ORDER BY last_played_at DESC
    LIMIT 1
),
target_member AS (
    SELECT
        members.*
    FROM members
    WHERE members.display_name_global = ?
    ORDER BY last_played_at DESC
    LIMIT 1
),
source_activities AS
(
    SELECT
        member_activities.*
    FROM member_activities
    INNER JOIN source_member ON member_activities.membership_id = source_member.membership_id
    WHERE member_activities.mode = ?
),
similiar_instances AS
(
    SELECT
        source_member.membership_id AS source_membership,
        target_member.membership_id AS target_membership,
        source_activities.instance_id
    FROM source_member
    INNER JOIN source_activities ON source_member.membership_id = source_activities.membership_id
    INNER JOIN member_activities ON
        source_activities.instance_id  = member_activities.instance_id
    INNER JOIN target_member ON member_activities.membership_id = target_member.membership_id
    GROUP BY  source_activities.membership_id, source_activities.instance_id
)
SELECT
    source_member.display_name_global,
    target_member.display_name_global,
    COUNT(DISTINCT similiar_instances.instance_id) AS same_instances
FROM source_member
INNER JOIN similiar_instances  ON source_member.membership_id = similiar_instances.source_membership
INNER JOIN target_member ON similiar_instances.target_membership = target_member.membership_id