use crate::database::account::Account;
use levelcrush::database;
use levelcrush::macros::DatabaseRecord;
use levelcrush::types::RecordId;
use levelcrush::util::unix_timestamp;
use sqlx::MySqlPool;

pub enum AccountPlatformType {
    Discord,
    Twitch,
    Bungie,
}

impl std::fmt::Display for AccountPlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountPlatformType::Discord => {
                write!(f, "discord")
            }
            AccountPlatformType::Twitch => {
                write!(f, "twitch")
            }
            AccountPlatformType::Bungie => {
                write!(f, "bungie")
            }
        }
    }
}

#[DatabaseRecord]
pub struct AccountPlatform {
    pub account: RecordId,
    pub token: String,
    pub platform: String,
    pub platform_user: String,
}

/// Required data inputs to generate a platform record
pub struct NewAccountPlatform {
    pub account: RecordId,
    pub platform: AccountPlatformType,
    pub platform_user: String,
}

/// Inserts a new accounts_platform record based on provided information.
pub async fn create(new_platform: NewAccountPlatform, pool: &MySqlPool) -> Option<AccountPlatform> {
    let token_seed = format!(
        "{}||{}||{}",
        new_platform.platform,
        new_platform.platform_user.clone(),
        unix_timestamp()
    );
    let token = format!("{:X}", md5::compute(token_seed));

    let query_result = sqlx::query_file!(
        "queries/account_platform_insert.sql",
        new_platform.account,
        token,
        new_platform.platform.to_string(),
        new_platform.platform_user,
        unix_timestamp()
    )
    .execute(pool)
    .await;

    // attempt to fetch the last inserted platform record
    if let Ok(query_result) = query_result {
        let last_inserted_id = query_result.last_insert_id();
        let platform_result = sqlx::query_file_as!(
            AccountPlatform,
            "queries/account_platform_get_by_id.sql",
            last_inserted_id
        )
        .fetch_optional(pool)
        .await;

        if let Ok(platform_result) = platform_result {
            platform_result
        } else {
            database::log_error(platform_result);
            None
        }
    } else {
        database::log_error(query_result);
        None
    }
}

/// fetches an account platform directly tied to the provided account and platform type
pub async fn from_account(
    account: &Account,
    platform_type: AccountPlatformType,
    pool: &MySqlPool,
) -> Option<AccountPlatform> {
    let query_result = sqlx::query_file_as!(
        AccountPlatform,
        "queries/account_platform_from_account.sql",
        account.id,
        platform_type.to_string()
    )
    .fetch_optional(pool)
    .await;

    if let Ok(query_result) = query_result {
        query_result
    } else {
        database::log_error(query_result);
        None
    }
}

/// Based off the provided platform information, attempts to match a platform login with an existing account
pub async fn match_account(
    platform_user: String,
    platform_type: AccountPlatformType,
    pool: &MySqlPool,
) -> Option<Account> {
    let query_result = sqlx::query_file_as!(
        Account,
        "queries/account_platform_match_account.sql",
        platform_type.to_string(),
        platform_user
    )
    .fetch_optional(pool)
    .await;

    if let Ok(query_result) = query_result {
        query_result
    } else {
        database::log_error(query_result);
        None
    }
}

/// read a platform record tied to the platform user, fetches the first created linked platform that matches the provided options
pub async fn read(
    platform_type: AccountPlatformType,
    platform_user: String,
    pool: &MySqlPool,
) -> Option<AccountPlatform> {
    let query_result = sqlx::query_file_as!(
        AccountPlatform,
        "queries/account_platform_read.sql",
        platform_type.to_string(),
        platform_user
    )
    .fetch_optional(pool)
    .await;

    if let Ok(query_result) = query_result {
        query_result
    } else {
        database::log_error(query_result);
        None
    }
}

/// Update the provied account platform record and returns a new updated account platform record
pub async fn update(account_platform: &mut AccountPlatform, pool: &MySqlPool) -> Option<AccountPlatform> {
    // force the platform record to have an updated timestamp of modification
    account_platform.updated_at = unix_timestamp();

    sqlx::query_file!(
        "queries/account_platform_update.sql",
        account_platform.account,
        account_platform.updated_at,
        account_platform.id
    )
    .execute(pool)
    .await
    .ok();

    let query = sqlx::query_file_as!(
        AccountPlatform,
        "queries/account_platform_get_by_id.sql",
        account_platform.id,
    )
    .fetch_optional(pool)
    .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

/// Unlink an account platfrom by directly deleting the related data tied to the account platform and then remove the account platform record itself as well
/// This is a permanent operation
pub async fn unlink(account_platform: &AccountPlatform, pool: &MySqlPool) {
    // remove the account platform data first
    sqlx::query_file!("queries/account_platform_data_unlink.sql", account_platform.id)
        .execute(pool)
        .await
        .ok();

    // remove the account platform now
    sqlx::query_file!("queries/account_platform_unlink.sql", account_platform.id)
        .execute(pool)
        .await
        .ok();
}
