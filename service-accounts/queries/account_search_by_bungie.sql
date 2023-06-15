WITH
    {}
SELECT
    accounts.token AS account_token,
    discord_platform_accounts.display_name AS discord,
    COALESCE(bungie_platform_accounts.display_name, '') AS bungie,
    COALESCE(twitch_platform_accounts.display_name, '') AS twitch
FROM bungie_platform_accounts
INNER JOIN accounts ON  bungie_platform_accounts.account = accounts.id
INNER JOIN discord_platform_accounts ON accounts.id = discord_platform_accounts.account # Every account must have a linked discord
LEFT JOIN twitch_platform_accounts ON accounts.id = twitch_platform_accounts.account
LIMIT 1