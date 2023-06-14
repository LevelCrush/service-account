use std::collections::HashMap;

use sqlx::MySqlPool;

use levelcrush::{database, util::unix_timestamp};
use levelcrush_macros::{project_path, DatabaseRecord, DatabaseResultSerde};

#[DatabaseResultSerde]
pub struct AccountLinkedPlatformsResult {
    pub account_token: String,
    pub discord: String,
    pub bungie: String,
    pub twitch: String,
}

#[DatabaseRecord]
pub struct Account {
    pub token: String,
    pub admin: i8,
    pub token_secret: String,
    pub timezone: String,
    pub last_login_at: u64,
}

pub async fn get<T: Into<String>, TS: Into<String>>(token: T, token_secret: TS, pool: &MySqlPool) -> Option<Account> {
    let query_result = sqlx::query_as!(
        Account,
        r"SELECT
            accounts.*
        FROM `levelcrush_accounts`.accounts AS accounts
        WHERE accounts.token = ?
        AND accounts.token_secret = ?
        LIMIT 1",
        token.into(),
        token_secret.into()
    );

    let query_result = query_result.fetch_optional(pool).await;

    if let Ok(query_result) = query_result {
        query_result
    } else {
        database::log_error(query_result);
        None
    }
}

/**
 * @brief Inserts and returns the account that is created based off the two provided seeds
 *
 * * `token_seed` Seed used to compute the public token identifier.
 * * `token_secret_seed` Seed used to compute the private token identifier
 */
pub async fn create<TokenSeed: Into<String>, TokenSecretSeed: Into<String>>(
    token_seed: TokenSeed,
    token_secret_seed: TokenSecretSeed,
    pool: &MySqlPool,
) -> Option<Account> {
    let token = format!("{:x}", md5::compute(token_seed.into()));
    let token_secret = format!("{:x}", md5::compute(token_secret_seed.into()));
    let timestamp = unix_timestamp();

    let query_result = sqlx::query!(
        r"
        INSERT INTO accounts
        SET
            id = NULL,
            token = ?,
            token_secret = ?,
            admin = 0,
            timezone = '',
            last_login_at = 0,
            created_at = ?,
            updated_at = 0,
            deleted_at = 0
    ",
        token,
        token_secret,
        timestamp
    )
    .execute(pool)
    .await;

    // if we were able to insert a new user fetch it based off the last inserted id from our query result
    let mut user = None;
    if let Ok(query_result) = query_result {
        let last_inserted_id = query_result.last_insert_id();
        let account_result = sqlx::query_as!(
            Account,
            r"
            SELECT
                accounts.*
            FROM accounts
            WHERE accounts.id = ?
            AND accounts.deleted_at = 0
            LIMIT 1
        ",
            last_inserted_id
        )
        .fetch_optional(pool)
        .await;

        if let Ok(account_result) = account_result {
            user = account_result;
        } else {
            database::log_error(account_result);
        }
    } else {
        database::log_error(query_result);
    }

    user
}

pub async fn all_data(account: &Account, pool: &MySqlPool) -> HashMap<String, HashMap<String, String>> {
    let query_results = sqlx::query!(
        r"
            SELECT
                account_platforms.platform,
                account_platforms.platform_user,
                account_platform_data.key,
                account_platform_data.value
            FROM account_platform_data
            INNER JOIN account_platforms ON account_platform_data.platform = account_platforms.id
            INNER JOIN accounts ON account_platform_data.account = accounts.id
            WHERE account_platform_data.account = ?
            ORDER BY account_platforms.platform ASC, account_platforms.id ASC, account_platform_data.key ASC
    ",
        account.id
    )
    .fetch_all(pool)
    .await;

    // loop through the data and construct a hashmap of those values aggregated
    let mut results = HashMap::new();
    if query_results.is_ok() {
        let query_results = query_results.unwrap();
        for record in query_results.into_iter() {
            //let index = format!("{}@{}", record.platform, record.platform_user);
            let index = record.platform;
            if !results.contains_key(&index) {
                results.insert(index.clone(), HashMap::new());
            }

            results.entry(index).and_modify(|item: &mut HashMap<String, String>| {
                item.insert(record.key, record.value);
            });
        }
    }
    results
}

fn with_bungie_account(search: bool, total_input: usize) -> String {
    let where_search = if search {
        if total_input > 1 {
            let prepared_pos = vec!["?"; total_input].join(",");
            format!("WHERE membership_data.value IN ({})", prepared_pos)
        } else {
            "WHERE membership_data.value = ?".to_string()
        }
    } else {
        String::new()
    };

    format!(
        r"
        bungie_platform_accounts AS (
            SELECT
                account_platforms.account AS account,
                membership_data.platform AS platform,
                membership_data.value AS display_name
            FROM account_platforms AS account_platforms
            INNER JOIN account_platform_data AS membership_data ON
                account_platforms.id = membership_data.platform AND
                account_platforms.account = membership_data.account AND
                account_platforms.platform = 'bungie' AND
                membership_data.key = 'unique_name'
            {}
        )
    ",
        where_search
    )
}

fn with_discord_account(search: bool, total_input: usize) -> String {
    let where_search = if search {
        if total_input > 1 {
            let prepared_pos = vec!["?"; total_input].join(",");
            format!("WHERE membership_data.value IN ({})", prepared_pos)
        } else {
            "WHERE membership_data.value = ?".to_string()
        }
    } else {
        String::new()
    };
    format!(
        r"
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
        ",
        where_search
    )
}

fn with_twitch_account(search: bool, total_input: usize) -> String {
    let where_search = if search {
        if total_input > 1 {
            let prepared_pos = vec!["?"; total_input].join(",");
            format!("WHERE membership_data.value IN ({})", prepared_pos)
        } else {
            "WHERE membership_data.value = ?".to_string()
        }
    } else {
        String::new()
    };

    format!(
        r"
        twitch_platform_accounts AS (
            SELECT
                account_platforms.account AS account,
                membership_data.platform AS platform,
                membership_data.value AS display_name
            FROM account_platforms AS account_platforms
            INNER JOIN account_platform_data AS membership_data ON
                account_platforms.id = membership_data.platform AND
                account_platforms.account = membership_data.account AND
                account_platforms.platform = 'twitch' AND
                membership_data.key = 'display_name'
            {}
        )
        ",
        where_search
    )
}

pub async fn by_bungie_bulk(bungie_ids: &[String], pool: &MySqlPool) -> Vec<AccountLinkedPlatformsResult> {
    let with_tables = vec![
        with_bungie_account(true, bungie_ids.len()),
        with_discord_account(false, 0),
        with_twitch_account(false, 0),
    ]
    .join(",");

    let statement = format!(
        r"
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
    ",
        with_tables
    );

    let mut query_builder = sqlx::query_as::<_, AccountLinkedPlatformsResult>(statement.as_str());
    for bungie_id in bungie_ids.iter() {
        query_builder = query_builder.bind(bungie_id);
    }

    let query = query_builder.fetch_all(pool).await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn by_bungie(bungie_id: String, pool: &MySqlPool) -> Option<AccountLinkedPlatformsResult> {
    let with_tables = vec![
        with_bungie_account(true, 1),
        with_discord_account(false, 0),
        with_twitch_account(false, 0),
    ]
    .join(",");

    let statement = format!(
        r"
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

    ",
        with_tables
    );

    let query = sqlx::query_as::<_, AccountLinkedPlatformsResult>(statement.as_str())
        .bind(bungie_id)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}
