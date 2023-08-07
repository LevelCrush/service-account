WITH
target_members AS (
    SELECT
        members.*
    FROM clans
    INNER JOIN clan_members ON clans.group_id = clan_members.group_id
    INNER JOIN members ON clan_members.membership_id = members.membership_id
    WHERE clans.is_network = 1
),
triumph_titles AS (
    SELECT
        triumphs.*
    FROM manifest_triumphs AS triumphs
    WHERE triumphs.is_title = 1
),
leaderboard AS (
    SELECT
        target_members.display_name_global      AS display_name,
        COALESCE(SUM(member_triumphs.state & 64 = 64), 0) AS amount
    FROM target_members
    INNER JOIN member_triumphs ON target_members.membership_id = member_triumphs.membership_id
    INNER JOIN triumph_titles ON member_triumphs.hash = triumph_titles.hash
    GROUP BY target_members.display_name_global
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
