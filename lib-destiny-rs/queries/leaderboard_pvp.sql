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
   WHERE member_activities.mode IN({}) /* PvP activities here */
   GROUP BY member_activities.instance_id, member_activities.membership_id
),
match_standings AS (
    SELECT
        target_activities.instance_id,
        target_activities.membership_id,
        standing_stat.value = 0 AS had_victory
    FROM target_activities
    INNER JOIN member_activity_stats AS standing_stat ON
           target_activities.instance_id = standing_stat.instance_id AND
           target_activities.membership_id = standing_stat.membership_id  AND
           standing_stat.name = 'standing'
    GROUP BY target_activities.instance_id, target_activities.membership_id,  standing_stat.value
),
leaderboard AS (
    SELECT
        target_members.display_name_global AS display_name,
        /*SUM(match_standings.had_victory = 1) / SUM(match_standings.had_victory = 0) AS wl_ratio, */ /* win/loss ratio */
        (SUM(match_standings.had_victory = 1) / COUNT(DISTINCT match_standings.instance_id) * 100) AS win_rate,
       /* SUM(match_standings.had_victory) AS wins,
        SUM(match_standings.had_victory = 0) AS losses, */
        COUNT(DISTINCT match_standings.instance_id) AS total_matches
    FROM target_members
    LEFT JOIN match_standings ON target_members.membership_id = match_standings.membership_id
    GROUP BY target_members.display_name_global, target_members.membership_id
),
leaderboard_standings AS (
    SELECT
        leaderboard.display_name,
        /*leaderboard.wl_ratio, */
        leaderboard.win_rate,
        /*leaderboard.wins,
        leaderboard.losses,
        leaderboard.total_matches, */
        (RANK() OVER w) AS `standing`,
        (PERCENT_RANK() OVER w) * 100 AS `percent_ranking`
    FROM leaderboard
    WHERE leaderboard.total_matches >= 100
    WINDOW w AS (ORDER BY leaderboard.win_rate DESC)
)

/* normalize expected output */
SELECT
    leaderboard_standings.display_name,
   /* leaderboard_standings.wl_ratio , */
    leaderboard_standings.win_rate AS amount,
    leaderboard_standings.standing,
    /*
    leaderboard_standings.wins,
    leaderboard_standings.losses,
    leaderboard_standings.total_matches, */
    leaderboard_standings.percent_ranking 
FROM leaderboard_standings
ORDER BY leaderboard_standings.standing ASC, leaderboard_standings.display_name ASC
