WITH
linked_bungies AS
(
  SELECT
        bungie_platform_data.account AS account,
        bungie_platform_data.platform AS platform,
        bungie_platform_data.value AS membership_id
  FROM `levelcrush_accounts`.account_platforms  AS account_platforms
  INNER JOIN `levelcrush_accounts`.`account_platform_data` AS bungie_platform_data ON
        account_platforms.account = bungie_platform_data.account  AND
        account_platforms.id  = bungie_platform_data.platform AND
        bungie_platform_data.key = 'primary_membership_id'
  WHERE account_platforms.platform = 'bungie'
),
linked_discords AS
(
    SELECT
        linked_bungies.account,
        discord_platform_data.platform,
        linked_bungies.membership_id,
        discord_platform_data.value AS discord_display_name
    FROM linked_bungies
    INNER JOIN `levelcrush_accounts`.account_platforms AS discord_platform ON
        linked_bungies.account = discord_platform.account AND
        discord_platform.platform = 'discord'
    INNER JOIN `levelcrush_accounts`.account_platform_data AS discord_platform_data ON
        discord_platform.account = discord_platform_data.account AND
        discord_platform.id = discord_platform_data.platform  AND
        discord_platform_data.key = 'display_name'
),
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
   WHERE member_activities.mode IN(39,41,42,84, 69,72,74,10,12,15,19,25,31,37,38,39,41,42,43,44,45,48,49,50,59,60,61,62,65,68,69,70,71,72,73,74,80,81,84,89,90,91) /*aids */
   GROUP BY member_activities.instance_id, member_activities.membership_id
),
match_standings AS (
    SELECT
        target_activities.instance_id,
        target_activities.membership_id,
        member_activity_stats.value = 0 AS had_victory,
        member_activity_stats.value >= 1 AS had_defeat
    FROM target_activities
    INNER JOIN member_activity_stats ON
           target_activities.instance_id = member_activity_stats.instance_id AND
           target_activities.membership_id = member_activity_stats.membership_id  AND
           member_activity_stats.name = 'standing'
    GROUP BY target_activities.instance_id, target_activities.membership_id, member_activity_stats.value_display, member_activity_stats.value
),

leaderboard AS (
    SELECT
        COALESCE(linked_discords.discord_display_name, target_members.display_name_global) AS display_name,
        SUM(match_standings.had_victory) / SUM(match_standings.had_defeat) AS wl_ratio, /* win/loss ratio */
        (SUM(match_standings.had_victory) / COUNT(DISTINCT match_standings.instance_id) * 100) AS win_rate,
        SUM(match_standings.had_victory) AS wins,
        SUM(match_standings.had_defeat) AS losses,
        COUNT(DISTINCT match_standings.instance_id) AS total_matches
    FROM target_members
    LEFT JOIN match_standings ON target_members.membership_id = match_standings.membership_id
    LEFT JOIN linked_bungies ON target_members.membership_id = linked_bungies.membership_id
    LEFT JOIN linked_discords ON linked_bungies.account = linked_discords.account
    GROUP BY target_members.display_name_global, target_members.membership_id, linked_discords.discord_display_name
),
leaderboard_standings AS (
    SELECT
        leaderboard.display_name,
        leaderboard.wl_ratio,
        leaderboard.win_rate,
        leaderboard.wins,
        leaderboard.losses,
        leaderboard.total_matches,
        (RANK() OVER w) AS `standing`,
        (CUME_DIST() OVER w)  * 100 AS `percent_distance`,
        (PERCENT_RANK() OVER w) * 100 AS `percent_ranking`
    FROM leaderboard
    WHERE leaderboard.total_matches >= 100
    WINDOW w AS (ORDER BY leaderboard.win_rate DESC)
)

/* normalize expected output */
SELECT
    leaderboard_standings.display_name,
    leaderboard_standings.wl_ratio + 0.0 AS amount, /* this seems silly, but is required for BigDecimal to be mapped as our uniform type */
    leaderboard_standings.win_rate,
    leaderboard_standings.standing,
    leaderboard_standings.wins,
    leaderboard_standings.losses,
    leaderboard_standings.total_matches,
    leaderboard_standings.percent_distance,
    leaderboard_standings.percent_ranking
FROM leaderboard_standings
ORDER BY leaderboard_standings.standing ASC, leaderboard_standings.display_name ASC
