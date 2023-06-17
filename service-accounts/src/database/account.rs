use levelcrush::{database, macros::DatabaseRecord, macros::DatabaseResultSerde, project_str, util::unix_timestamp};
use sqlx::MySqlPool;
use std::collections::HashMap;

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
    let token = token.into();
    let token_secret = token_secret.into();

    let query_result = sqlx::query_file_as!(Account, "queries/account_get_by_token.sql", token, token_secret)
        .fetch_optional(pool)
        .await;

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

    let query_result = sqlx::query_file!("queries/account_insert.sql", token, token_secret, timestamp)
        .execute(pool)
        .await;

    // if we were able to insert a new user fetch it based off the last inserted id from our query result
    let mut user = None;
    if let Ok(query_result) = query_result {
        let last_inserted_id = query_result.last_insert_id();
        let account_result = sqlx::query_file_as!(Account, "queries/account_get_by_id.sql", last_inserted_id)
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
    let query_results = sqlx::query_file!("queries/account_platform_all_data.sql", account.id)
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

    project_str!("queries/account_search.with.bungie.sql", where_search)
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
    project_str!("queries/account_search.with.discord.sql", where_search)
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

    project_str!("queries/account_search.with.twitch.sql", where_search)
}

pub async fn by_bungie_bulk(bungie_ids: &[String], pool: &MySqlPool) -> Vec<AccountLinkedPlatformsResult> {
    let with_tables = vec![
        with_bungie_account(true, bungie_ids.len()),
        with_discord_account(false, 0),
        with_twitch_account(false, 0),
    ]
    .join(",");

    let statement = project_str!("queries/account_search_by_bungie_bulk.sql", with_tables);
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

    let statement = project_str!("queries/account_search_by_bungie.sql", with_tables);

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

pub async fn by_discord(discord_id: String, pool: &MySqlPool) -> Option<AccountLinkedPlatformsResult> {
    let with_tables = vec![
        with_bungie_account(false, 0),
        with_discord_account(true, 1),
        with_twitch_account(false, 0),
    ]
    .join(",");

    let statement = project_str!("queries/account_search_by_discord.sql", with_tables);
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
