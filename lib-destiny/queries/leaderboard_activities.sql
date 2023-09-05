WITH
target_members AS (
    SELECT
        members.*
    FROM clans
    INNER JOIN clan_members ON clans.group_id = clan_members.group_id
    INNER JOIN members ON clan_members.membership_id = members.membership_id
    WHERE clans.is_network = 1
),
target_activities AS
(
   SELECT
       member_activities.instance_id,
       member_activities.membership_id
   FROM target_members
   INNER JOIN member_activities ON target_members.membership_id = member_activities.membership_id
   WHERE member_activities.mode IN({})
   GROUP BY member_activities.instance_id, member_activities.membership_id
),
full_clear_activities AS
(
    SELECT
        instances.instance_id,
        target_activities.membership_id
    FROM target_activities
    INNER JOIN instances ON target_activities.instance_id = instances.instance_id
    INNER JOIN instance_members ON
        target_activities.membership_id = instance_members.membership_id AND
        target_activities.instance_id = instance_members.instance_id
    WHERE instance_members.completed = 1
    AND instances.completed = 1
    AND instances.completion_reasons = 'Objective Completed'
    GROUP BY instances.instance_id, target_activities.membership_id
),

leaderboard AS (
    SELECT
        target_members.display_name_global AS display_name,
        COUNT(DISTINCT full_clear_activities.instance_id) AS amount
    FROM target_members
    LEFT JOIN full_clear_activities ON target_members.membership_id = full_clear_activities.membership_id
    GROUP BY target_members.display_name_global, target_members.membership_id
),
leaderboard_standings AS (
    SELECT
        leaderboard.display_name,
        leaderboard.amount,
        (RANK() OVER w) AS `standing`,
        (PERCENT_RANK() OVER w) * 100 AS `percent_ranking`
    FROM leaderboard
    WINDOW w AS (ORDER BY leaderboard.amount DESC)
)

/* normalize expected output */
SELECT
    leaderboard_standings.display_name,
    leaderboard_standings.amount + 0.0 AS amount, /* this seems silly, but is required for BigDecimal to be mapped as our uniform type */
    leaderboard_standings.standing,
    leaderboard_standings.percent_ranking
FROM leaderboard_standings
ORDER BY leaderboard_standings.standing ASC, leaderboard_standings.display_name ASC
