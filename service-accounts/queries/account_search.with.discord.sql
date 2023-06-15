discord_platform_accounts AS (
    SELECT
        account_platforms.account AS account,
        membership_data.platform AS platform,
        membership_data.value AS display_name
    FROM account_platforms AS account_platforms
    INNER JOIN account_platform_data AS membership_data ON
        account_platforms.id = membership_data.platform AND
        account_platforms.account = membership_data.account AND
        account_platforms.platform = 'discord' AND
        membership_data.key = 'display_name'
    {}
)