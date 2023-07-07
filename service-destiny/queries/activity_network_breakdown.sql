WITH target_clan AS (
    SELECT
        clans.*
    FROM clans
    WHERE clans.is_network = 1
),
target_clan_members AS (
    SELECT
        clan_members.*
    FROM clan_members
    INNER JOIN target_clan ON clan_members.group_id = target_clan.group_id
),
target_instances AS (
    SELECT
        member_activities.membership_id,
        member_activities.instance_id
    FROM member_activities
    INNER JOIN target_clan_members ON member_activities.membership_id = target_clan_members.membership_id
    WHERE member_activities.mode IN ({})
    AND member_activities.occurred_at >= ? AND member_activities.occurred_at <= ?
    GROUP BY member_activities.membership_id, member_activities.instance_id
),
target_instances_clan_member_count AS (
    SELECT
        target_instances.instance_id,
        COUNT(DISTINCT target_clan_members.membership_id) AS clan_members
    FROM target_instances
    INNER JOIN instance_members ON target_instances.instance_id = instance_members.instance_id
    LEFT JOIN target_clan_members ON instance_members.membership_id = target_clan_members.membership_id
    WHERE instance_members.completed = 1
    GROUP BY target_instances.instance_id
),
target_completed_instances AS (
    SELECT
        target_instances.instance_id
    FROM target_instances
    INNER JOIN instance_members ON target_instances.instance_id = instance_members.instance_id
    INNER JOIN target_clan_members ON instance_members.membership_id = target_clan_members.membership_id
    WHERE instance_members.completed = 1
    GROUP BY target_instances.instance_id
),
target_instances_with_clan AS (
    SELECT
        target_instances.instance_id,
        COUNT(DISTINCT target_clan_members.membership_id) AS clan_members
    FROM target_instances
    INNER JOIN instance_members ON target_instances.instance_id = instance_members.instance_id
    INNER JOIN target_clan_members ON instance_members.membership_id = target_clan_members.membership_id AND instance_members.membership_id != target_instances.membership_id
    WHERE instance_members.completed = 1
    GROUP BY target_instances.instance_id
),

clan_breakdown AS (
    SELECT
        target_clan.group_id,
        target_clan.name,
        COUNT(DISTINCT target_clan_members.membership_id)           AS total_members,
        COUNT(DISTINCT target_instances.instance_id)                AS activity_attempts,
        COUNT(DISTINCT target_completed_instances.instance_id)      AS activities_completed,
        COUNT(DISTINCT target_instances_with_clan.instance_id)      AS activities_completed_with_clan,
        COALESCE(ROUND(AVG(target_instances_clan_member_count.clan_members)),0) AS avg_clan_member_amount
    FROM target_clan
    INNER JOIN target_clan_members ON target_clan.group_id = target_clan_members.group_id
    LEFT JOIN target_instances ON target_clan_members.membership_id = target_instances.membership_id
    LEFT JOIN target_completed_instances ON target_instances.instance_id = target_completed_instances.instance_id
    LEFT JOIN target_instances_with_clan ON target_instances.instance_id = target_instances_with_clan.instance_id
    LEFT JOIN target_instances_clan_member_count ON target_instances.instance_id = target_instances_clan_member_count.instance_id
    GROUP BY target_clan.name
)


SELECT
    clan_breakdown.group_id,
    clan_breakdown.name,
    clan_breakdown.total_members,
    clan_breakdown.activity_attempts,
    clan_breakdown.activities_completed_with_clan,
    COALESCE(ROUND((clan_breakdown.activities_completed_with_clan / clan_breakdown.activities_completed) * 100 ),0) AS percent_with_clan,
    clan_breakdown.activities_completed,
    clan_breakdown.avg_clan_member_amount
FROM clan_breakdown