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
triumph_titles AS (
    SELECT
        triumphs.*
    FROM triumphs
    WHERE triumphs.is_title = 1
),
leaderboard AS (
    SELECT
        COALESCE(linked_discords.discord_display_name, target_members.display_name_global)      AS display_name,
        COALESCE(SUM(member_triumphs.state & 64 = 64), 0) AS amount
    FROM target_members
          INNER JOIN member_triumphs ON target_members.membership_id = member_triumphs.membership_id
          INNER JOIN triumph_titles ON member_triumphs.hash = triumph_titles.hash
          LEFT JOIN linked_bungies ON target_members.membership_id = linked_bungies.membership_id
          LEFT JOIN linked_discords ON linked_bungies.account = linked_discords.account
    GROUP BY target_members.display_name_global, linked_discords.discord_display_name
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
